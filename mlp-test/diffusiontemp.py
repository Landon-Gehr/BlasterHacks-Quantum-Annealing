import numpy as np
import torch

##############
# scheduling #
##############

def lin_schedule(T, start=1e-4,stop=2e-2, **kwargs):
    betas = torch.linspace(start, stop, T)
    alphas = 1 - betas
    alpha_bars = torch.Tensor(np.cumprod(alphas))
    return betas, alphas, alpha_bars

###################
# forward process #
###################

def noise_data(x_0, num_skip, alpha_bars, max_steps, device, use_max=False):
    B = x_0.shape[0] # (batch size x n features)
    if x_0.device != device:
        x_0 = x_0.to(device=device) # ensure data is on correct device
    if use_max:
        t = torch.full((B,),max_steps-1,device=device,dtype=torch.long)
    else:
        t = torch.randint(0, max_steps, (B,), device=device) # t ~ U(0, T), vector (1 x batchesize)
    ep_shape = (x_0.shape[0], x_0.shape[1] - num_skip)
    epsilon = torch.randn(ep_shape, device=device) # e ~ N(0, I)

    # sqrt(alpha_bar) x_0 + sqrt(1 - alpha_bar) e
    x_0_coef = torch.sqrt(alpha_bars[t]).reshape(-1, 1).to(device=device) # reshape into column vec of (batchsize x 1)
    epsilon_coef = torch.sqrt(1 - alpha_bars[t]).reshape(-1, 1).to(device=device)
    x_t = x_0_coef * x_0[:,num_skip:] + epsilon_coef * epsilon
    x_t = torch.cat([x_0[:, :num_skip], x_t], dim=1)
    epsilon = torch.nn.functional.pad(epsilon, (num_skip, 0))

    return x_t, t, epsilon


###################
# reverse process #
###################

def unnoise_data(x_t):
    t_tensor = torch.empty(x_t.shape[0], dtype=torch.long, device=model.device)
    for t in range(T-1, -1, -1): 
        mean_eps = (1 - alphas[t]) / torch.sqrt(1 - alpha_bars[t])
        var_eps = 1 / torch.sqrt(alphas[t])
        dev_eps = torch.sqrt(betas[t])

        t_tensor.fill_(t)
        z =  torch.randn_like(x_t) if t > 1 else torch.zeros_like(x_t)
        epsilon_theta_hat = model(x_t, t_tensor)
        x_t_1  = (var_eps * (x_t - mean_eps * epsilon_theta_hat) + dev_eps * z)
        x_t = x_t_1
    return x_t



#################
# training loop #
#################

# https://arxiv.org/pdf/2006.11239#page=4
# 1.) take batch sample
# 2.) select random timestep
# 3.) apply known noise to sample 
# 4.) get model prediction of error
# 5.) take gradient step on squared difference between predicted and actual error 
def train(model, trainloader, valloader):
    epoch_val_loss = np.inf
    epoch_train_loss = np.inf
    for epoch in range(n_epochs):
        print(f"epoch: {epoch}/{n_epochs}, (train, val) loss: {epoch_train_loss:.7f}, {epoch_val_loss:.7f}",flush=True)
       
        batch_losses = []

        ############
        # training #
        ############
        model.train()
        for batch_n, (x_0,) in enumerate(trainloader): # take batch sample, no labels only inputs
            B = x_0.shape[0] # batch size
            x_0 = x_0.to(device=model.device) #ensure data is on same device as model (should be gpu)

            x_t, t, epsilon = noise_data(x_0)

            epsilon_hat = model(x_t,t)

            loss = torch.nn.functional.mse_loss(epsilon_hat, epsilon)
            
            loss.backward() # calculate the gradient (is cumulative)
            optimizer.step() # step in direction of gradient

            batch_losses.append(loss.item())
        epoch_train_loss = np.mean(batch_losses)

        ####################
        # Validation error #
        ####################
        batch_losses = []
        model.eval()
        for idx, (x_0,) in enumerate(valloader):
            x_t, t, epsilon = noise_data(x_0)
            with torch.no_grad():
                epsilon_hat = model(x_t,t)
                loss = torch.nn.functional.mse_loss(epsilon_hat, epsilon)
            batch_losses.append(loss.item())
        epoch_val_loss = (np.mean(batch_losses))

        ####################################
        # val loss evaluation for stopping #
        ####################################

        if best_loss > epoch_val_loss:
            best_loss = epoch_val_loss
            torch.save(model.state_dict(), f"model.pt")





############
# sampling #
############

# noises an original data and performs reverse process on that
# https://arxiv.org/pdf/2006.11239#page=4
def inference(model, config, n_samples, reps_per_sample, dataloader, temperature=1.0, verbose=False, aggregate=False):
    schedule = get_schedules_dict()[config['diffusion']['schedule']]
    schedule_range = get_schedule_ranges_dict(config['diffusion']['schedule'])[config['diffusion']['schedule_range']]
    T = config["diffusion"]["num_timesteps"]
    n_unnoised = config['data']['n_unnoised_features']
    betas, alphas, alpha_bars = schedule(T, **schedule_range)    
    betas = betas.to(model.device)
    alphas = alphas.to(model.device)
    alpha_bars = alpha_bars.to(model.device)

    if len(dataloader.dataset) < n_samples:
        if verbose:
            print(f"WARNIG: {n_samples} is more samples than length of dataset ({len(dataloader.dataset)}), reducing nummber of samples taken",flush=True)
        n_samples = len(dataloader.dataset)

    total = n_samples if aggregate else n_samples * reps_per_sample
    samps = torch.zeros((total,config["data"]["n_total_features"]), device=model.device)
    originals = torch.zeros((total,config["data"]["n_total_features"]), device=model.device)
    with torch.no_grad():
        count = 0
        for i, (a,) in enumerate(dataloader):

            x_T, tvec, epsilon = noise_data(a, n_unnoised, alpha_bars, T, model.device, use_max = True)
            x_t = x_T.repeat_interleave(reps_per_sample,dim=0)
            x_t = unnoise_data(x_t, n_unnoised, betas, alphas, alpha_bars, T, model, temperature=temperature) 

            if aggregate:
                B = a.shape[0]
                x_t = x_t.view(B, reps_per_sample, x_t.shape[1]).mean(dim=1)
            else:
                B = a.shape[0]*reps_per_sample
                a = a.repeat_interleave(reps_per_sample,dim=0)
            left = min(B, total - count)
            samps[count:count+left] = x_t[:left]
            originals[count:count+left] = a[:left]
            count += left

            if verbose:
                print(f"batch {i} count {count} of {total}",flush=True)
            
            if count >= total:
                break

    return samps, originals


















########################################################################################################################



import time
import torch
import torchvision
import numpy as np
import matplotlib.pyplot as plt
from mnist_model_12_03_25 import Model
import sys
import os
import optuna

#################
# training loop #
#################

# https://arxiv.org/pdf/2006.11239#page=4
def train(model, config, optimizer, trainloader, testloader, T, alpha_bars):
    n_epochs = config["runner"]["n_epochs"]
    logging_steps = config["runner"]["logging_steps"]
    best_loss = np.inf
    train_loss = np.inf
    n_patience = config['runner']['patience']
    n_since_best = 0
    test_loss = np.inf
    epoch_train_loss = 0

    losses = []
    test_losses = []
    model.train()
    for epoch in range(n_epochs):
        print(f"epoch {epoch}/{n_epochs}: patience {n_since_best}/{n_patience}")
        train_loss = 0.0

        start_time = time.time()
        for batch_idx, (x_0, _) in enumerate(trainloader): # x_0 ~ q(x_0)
            # print(f"batch: {batch_idx+1}/{len(trainloader)}",flush=True)
            # if batch_idx > 10:
            #     break
            B = x_0.shape[0] # batch size
            x_0 = x_0.to(device=model.device) # ensure data is on correct device
            t = torch.randint(0, T, (B,)) # t ~ U(0, T)
            epsilon = torch.randn(x_0.shape, device=model.device) # e ~ N(0, I)
            
            # sqrt(alpha_bar) x_0 + sqrt(1 - alpha_bar) e
            x_0_coef = torch.sqrt(alpha_bars[t]).reshape(-1, 1, 1, 1).to(device=model.device)
            epsilon_coef = torch.sqrt(1 - alpha_bars[t]).reshape(-1, 1, 1, 1).to(device=model.device)
            x_t = x_0_coef * x_0 + epsilon_coef * epsilon

            # get models prediction of the error
            epsilon_theta = model(x_t.to(model.device), t.to(model.device))

            # Calculate loss as MSE
            loss = torch.sum(((epsilon - epsilon_theta)**2)/B)

            # gradient descent step in nn model
            loss.backward() # calculate the gradient (is cumulative)
            optimizer.step() # step in direction of gradient
            optimizer.zero_grad() # reset gradient to prevent accumulation

            train_loss += loss.item()
            epoch_train_loss += train_loss
            if (batch_idx + 1) % logging_steps == 0 :
                print(f'(Training, Test) Loss over last {logging_steps}({batch_idx+1}/{len(trainloader)}:{B*len(trainloader)}) batches: {(train_loss / (logging_steps * B)):.4f}, {test_loss:.4f} | Time (s): {(time.time() - start_time):.4f}',flush=True)
                train_loss = 0.0
                start_time = time.time()
        
        losses.append(train_loss/(len(trainloader)))
        epoch_train_loss = 0

            # all batches iterated, end epoch
        # check patience and test error 

        n_since_best +=1
        if best_loss > test_loss:
            best_loss = test_loss
            n_since_best = 0
            torch.save(model.state_dict(), os.path.join("models",config['model']['name']+".pt"))
        if n_since_best > n_patience:
            print("patience exceeded",flush=True)
            break
        test_loss = 0
        for idx, (x_0, _) in enumerate(testloader):
            # print(f"batch: {idx+1}/{len(testloader)}",flush=True)
            # if idx > 10:
            #     break
            B = x_0.shape[0]
            x_0 = x_0.to(device=model.device)
            t = torch.randint(0, T, (B,))
            epsilon = torch.randn(x_0.shape, device=model.device)
            x_0_coef = torch.sqrt(alpha_bars[t]).reshape(-1, 1, 1, 1).to(device=model.device)
            epsilon_coef = torch.sqrt(1 - alpha_bars[t]).reshape(-1, 1, 1, 1).to(device=model.device)
            x_t = x_0_coef * x_0 + epsilon_coef * epsilon
            epsilon_theta = model(x_t.to(device=model.device), t.to(device=model.device))
            loss = torch.sum(((epsilon - epsilon_theta)**2)/B)
            test_loss += loss.item()
        
        test_loss = test_loss/(len(testloader))
        test_losses.append(test_loss)

    return model, losses, test_losses


# https://arxiv.org/pdf/2006.11239   # diffusion paper 

import math
import numpy as np
import torch
import torch.nn as nn

# place into an embedding based on positon in time series #https://proceedings.neurips.cc/paper_files/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf
#       mapping every position into a higher embedding_dim dimensional space
# takes time step vector and outputs embeddin matrix where every row is the position in the time series and every column is a mapped index of 0<i<d
#       where the embedding_dimenesion d is the number of columns to include in the output, a hyperparamter
def positional_time_embedding(timesteps, embedding_dim, embedding_scale=10000): 

    #PE_even = sin(pos/(1e5)^(2i/embedding_dim))
    #PE_odd = cos(pos/(1e5)^(2i/embedding_dim))
        #creates 2 entries for every i, i should only range 0-(d/2)

    out_embedding = torch.zeros(len(timesteps), embedding_dim, dtype=torch.float32)
    out_embedding = out_embedding.to(device=timesteps.device)
    for pos in range(len(timesteps)):
        for i in range(embedding_dim//2):
            deno = np.pow(embedding_scale,2*i/embedding_dim)
            out_embedding[pos,2*i] = np.sin(pos/deno)
            out_embedding[pos,2*i+1] = np.cos(pos/deno)

    return out_embedding

# embed using period of random noise #https://medium.com/@j.calzaretta.ai/exploring-diffusion-models-a-hands-on-approach-with-mnist-baf79aa4d195
def gaussian_time_embedding(timesteps, embedding_dim, embedding_scale=30):    
    weights = torch.randn(embedding_dim// 2) * embedding_scale
    weights = weights.to(device=timesteps.device)
    out_embedding = timesteps[:, None] * weights[None, :] * 2 * math.pi
    out_embedding = torch.cat([torch.sin(out_embedding), torch.cos(out_embedding)], dim=-1)
    if (embedding_dim % 2 == 1): # odd // 2 will be 1 column short of embedding_dim
        out_embedding = torch.nn.functional.pad(out_embedding, (0, 1, 0, 0)) # pad right most column with zeros
    return out_embedding


# apply activation function, use mode to index as a hyperparameter
def get_time_embedding(timesteps,embedding_dim, mode, embedding_scale = None):
    embeddings = [positional_time_embedding, gaussian_time_embedding]
    if embedding_scale != None:
        return embeddings[mode](timesteps,embedding_dim,embedding_scale)
    return (embeddings[mode])(timesteps,embedding_dim)



# apply activation function, use mode to index as a hyperparameter
def activation(x, mode):
    activations = [torch.nn.functional.silu, torch.nn.functional.relu]
    return (activations[mode])(x)


def Normalize(in_channels):
    return torch.nn.GroupNorm(num_groups=32, num_channels=in_channels, eps=1e-6, affine=True)


class Upsample(nn.Module):
    def __init__(self, in_channels, with_conv, extra_pad):
        super().__init__()
        self.with_conv = with_conv
        self.extra_pad = extra_pad
        if self.with_conv: # optionally apply a 2d convolution layer
            self.conv = torch.nn.Conv2d(in_channels,in_channels,kernel_size=3,stride=1,padding=1)

    def forward(self, x): # nearest neighbor interpolation doubles image size by turning 1 pixel into 2x2 in upsampled image
        x = torch.nn.functional.interpolate(x, scale_factor=2.0, mode="nearest")
        if self.with_conv:
            x = self.conv(x)
        if self.extra_pad > 0:
            x = torch.nn.functional.pad(x, (0, self.extra_pad, 0, self.extra_pad), mode="replicate")
            # x = torch.nn.functional.pad(x, (0, self.extra_pad, 0, self.extra_pad), mode="constant",value=0)
        return x


class Downsample(nn.Module):
    def __init__(self, in_channels, mode):
        super().__init__()
        self.mode = mode
        def conv_pool(x, kernel_size, stride):
            convp = torch.nn.Conv2d(in_channels,in_channels,kernel_size=3,stride=2,padding=1).to(device=x.device)
            # x = torch.nn.functional.pad(x, (0, 1, 0, 1), mode="constant", value=0)
            return convp(x)

        self.poolings = [torch.nn.functional.avg_pool2d, torch.nn.functional.max_pool2d, conv_pool]

    def forward(self, x): 
        x = self.poolings[self.mode](x, kernel_size=2, stride=2)
        return x


class ResnetBlock(nn.Module):
    def __init__(self, *, num_layers, in_channels, out_channels=None, dropout, temb_channels=512, kernel_size=3, activation_mode, dim_matching_mode = 1):
        super().__init__()
        self.in_channels = in_channels
        self.out_channels = in_channels if out_channels is None else out_channels
        self.kernel = kernel_size
        self.pad = kernel_size//2
        self.activation_mode = activation_mode
        self.num_layers = num_layers
        self.dim_matching_mode = dim_matching_mode

        self.convs = nn.ModuleList()
        # self.norms = [] # this prevents sending to gpu?
        self.norms = nn.ModuleList()
        
        if num_layers - 2 > 0:
            for i in range(num_layers-2):
                self.norms.append(Normalize(in_channels))
                self.convs.append(torch.nn.Conv2d(in_channels,in_channels,kernel_size=self.kernel,stride=1,padding=self.pad))
            self.norms.append(Normalize(in_channels))
            self.convs.append(torch.nn.Conv2d(in_channels,out_channels,kernel_size=self.kernel,stride=1,padding=self.pad))
        else:
            self.norms.append(Normalize(in_channels))
            self.convs.append(torch.nn.Conv2d(in_channels,out_channels,kernel_size=self.kernel,stride=1,padding=self.pad))

        self.normsout = Normalize(out_channels)
        self.temb_proj = torch.nn.Linear(temb_channels,out_channels) # scale time embedding to right dimension
        self.dropout = torch.nn.Dropout(dropout)
        self.convout = torch.nn.Conv2d(out_channels,out_channels,kernel_size=self.kernel,stride=1,padding=self.pad)
        if self.dim_matching_mode:
            self.dim_match = torch.nn.Conv2d(in_channels,out_channels,kernel_size=1,stride=1,padding=0)
        else:
            self.dim_match = torch.nn.Conv2d(in_channels,out_channels,kernel_size=kernel_size,stride=1,padding=0)        

    def forward(self, x, temb):
        h = x # remember original input to apply at end

        for i in range(self.num_layers - 1):
            # print(f"passing {h.shape} into {self.norms[i]} from {self.norms}",flush=True)
            # print(f"from {self}",flush=True)
            h = self.norms[i](h)
            h = activation(h, self.activation_mode) 
            # print(f"passing {h.shape} into {self.convs[i]}",flush=True)
            h = self.convs[i](h)

        h = h + self.temb_proj(activation(temb, self.activation_mode))[:, :, None, None] # add on time embedding

        h = self.normsout(h)
        h = activation(h, self.activation_mode)
        h = self.dropout(h) # randomly set some elements to zero
        h = self.convout(h)

        x = self.dim_match(x)

        return x+h
    

class AttnBlock(nn.Module):
    def __init__(self, in_channels):
        super().__init__()
        self.in_channels = in_channels

        self.norm = Normalize(in_channels)
        # keys, queries, values matrices are learned matrices that produce keys and queries based on the data
        self.q = torch.nn.Conv2d(in_channels,in_channels, kernel_size=1, stride=1, padding=0)
        self.k = torch.nn.Conv2d(in_channels,in_channels,kernel_size=1,stride=1,padding=0)
        self.v = torch.nn.Conv2d(in_channels,in_channels,kernel_size=1,stride=1,padding=0)
        self.proj_out = torch.nn.Conv2d(in_channels,in_channels,kernel_size=1,stride=1,padding=0)

    def forward(self, x):
        h = x # remember original to add back
        h = self.norm(h)
        queries = self.q(h) # [x.shape]
        keys = self.k(h) # [x.shape]
        values = self.v(h) # [x.shape]

        # https://proceedings.neurips.cc/paper_files/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf#page=4
        # compute attention
        batches, dim_1, dim_2, dim_3 = queries.shape # [batches x c X g X w]
        queries = queries.reshape(batches, dim_1, dim_2*dim_3)
        queries = queries.permute(0, 2, 1)   # transpose without affect batches dim --> [b,gw,c]
        keys = keys.reshape(batches, dim_1, dim_2*dim_3)  # keep keys untransposed for matrix multiplication --> [b,c,gw]
        weights = torch.bmm(queries, keys)     # [b,gw,gw] --> batch matrix multiplication --> Q*K^t
        weights = weights * (int(dim_1)**(-0.5)) # Q*K^t / sqrt(d_k)
        weights = torch.nn.functional.softmax(weights, dim=2) # softmax(Q*K^t / sqrt(d_k))
        values = values.reshape(batches, dim_1, dim_2*dim_3)
        weights = weights.permute(0, 2, 1)
        h = torch.bmm(values, weights) # w = softmax(Q*K^t / sqrt(d_k)) --> softmax(Q*K^t / sqrt(d_k)) * V
        h = h.reshape(batches, dim_1, dim_2, dim_3)

        h = self.proj_out(h)

        return x+h



class Model(nn.Module):
    def __init__(self, config):
        super().__init__()

        ####################
        # unpacking config #
        ####################
        self.config = config #dictionary with input parameters

        #channels = depth/num layers of image data
        self.in_channels = self.config["model"]["in_channels"] #num of channels in first input layer
        self.out_channels = self.config["model"]["out_channels"] #num channels in final output layer
        self.channels_multiplier = self.config["model"]["channels_multiplier"] #list that determines how many times to increase num channels during hidden layer passes
        self.num_resolutions = self.config["model"]["num_updown_sampling"] # number of down/upsampling layers
        self.t_emb_dim = self.config["model"]["embedding_dim"] # the embedding dim in time higher dimesion projection
        self.t_dim_expansion = self.t_emb_dim*4
        self.embedding_mode = self.config["model"]["embedding_mode"]

        self.activation_mode = self.config["model"]["activation_mode"]
        self.kernel_sizes = self.config["model"]["kernel_sizes"]

        self.upsample_conv = self.config["model"]["upsample_conv"]
        self.downsample_mode = self.config["model"]["downsample_mode"]
        self.resolution_skips = self.config["model"]["resolution_skips"]

        self.dropout_percent = self.config["model"]["dropout"] # percent chance to set values to zero in dropout layers
        self.num_res_blocks = self.config["model"]["num_res_blocks"] # how many blocks with skips to use in a res net layer
        self.num_res_layers = self.config["model"]["layers_per_res_block"]
        self.attn_resolutions = self.config["model"]["attn_resolutions"] 

        self.image_resolution = self.config["data"]["image_size"] # num pixels the image is across (assuming square images)
        
        self.num_timesteps = self.config["diffusion"]["num_timesteps"] # number of steps in diffusion forward/backward process
        self.resos = [self.image_resolution]
        self.kernel_counter = 0
        self.attn_counter = 0

        ########################################
        # Start: embed input and initial layer #
        ########################################

        # passed through non convolutional layer (functionally equivalent to convolutional layer with kernel size 1)
        # print("input pass:",flush=True)
        # print("attns:",self.attn_resolutions,flush=True)
        self.temb = nn.Module()
        self.temb.dense = nn.ModuleList([torch.nn.Linear(self.t_emb_dim,self.t_dim_expansion),      # take time embedded data and scale dimension
                                         torch.nn.Linear(self.t_dim_expansion,self.t_dim_expansion),])
        
        self.conv_in = torch.nn.Conv2d(self.in_channels,self.t_emb_dim,kernel_size=self.kernel_sizes[self.kernel_counter],stride=1,padding=self.kernel_sizes[self.kernel_counter]//2)
        self.kernel_counter+=1
        # print("kernel:",self.kernel_counter,flush=True)


        ##############################
        # U net down sampling layers #
        ##############################

        self.down = nn.ModuleList()
        block_in_channels = self.t_emb_dim
        # print(f"start {self.num_resolutions}x{self.num_res_blocks} downsamples; kernels {self.kernel_counter+1}-{self.kernel_counter+1 + self.num_resolutions*self.num_res_blocks}",flush=True)
        for resolution_level in range(self.num_resolutions):
            block = nn.ModuleList()
            block_out_channels = block_in_channels * self.channels_multiplier[resolution_level] # double depth
            for resnet_block in range(self.num_res_blocks):
                block.append(ResnetBlock(num_layers=self.num_res_layers,
                                         in_channels=block_in_channels,
                                         out_channels=block_out_channels,
                                         temb_channels=self.t_dim_expansion,
                                         dropout=self.dropout_percent, 
                                         kernel_size=self.kernel_sizes[self.kernel_counter], 
                                         activation_mode=self.activation_mode["downsampling"]))
                self.kernel_counter+=1
                # print("kernel:",self.kernel_counter,flush=True)

                block_in_channels = block_out_channels
            down = nn.Module()
            if self.attn_resolutions[self.attn_counter]:
                down.attn = AttnBlock(block_out_channels)
            self.attn_counter += 1
            # print("attn:",self.attn_counter,flush=True)
            down.block = block
            down.downsample = Downsample(block_out_channels, self.downsample_mode) # halve resolution
            self.down.append(down)
            self.resos.append(self.resos[-1]//2)

        #####################
        # U net bottom pass #
        #####################

        # print(f"2 block bottom pass",flush=True)
        self.mid = nn.Module()
        self.mid.block_1 = ResnetBlock(num_layers=self.num_res_layers,
                                       in_channels=block_in_channels,
                                       out_channels=block_in_channels,
                                       temb_channels=self.t_dim_expansion,
                                       dropout=self.dropout_percent, 
                                       kernel_size=self.kernel_sizes[self.kernel_counter], 
                                       activation_mode=self.activation_mode["bottom"])
        self.kernel_counter+=1
        # print("kernel:",self.kernel_counter,flush=True)
        if self.attn_resolutions[self.attn_counter]:
            self.mid.attn = AttnBlock(block_in_channels)
        self.attn_counter += 1
        # print("attn:",self.attn_counter,flush=True)
        self.mid.block_2 = ResnetBlock(num_layers=self.num_res_layers,
                                       in_channels=block_in_channels,
                                       out_channels=block_in_channels,
                                       temb_channels=self.t_dim_expansion,
                                       dropout=self.dropout_percent, 
                                       kernel_size=self.kernel_sizes[self.kernel_counter], 
                                       activation_mode=self.activation_mode["bottom"])
        self.kernel_counter+=1
        # print("kernel:",self.kernel_counter,flush=True)

        
        ############################
        # U net up sampling layers #
        ############################
        # print(f"start {self.num_resolutions}x{self.num_res_blocks} upsamples; kernels {self.kernel_counter+1}-{self.kernel_counter+1 + self.num_resolutions*self.num_res_blocks}",flush=True)
        self.up = nn.ModuleList()
        for resolution_level in reversed(range(self.num_resolutions)):
            skip_con_channels = 0
            if self.resolution_skips:
                skip_con_channels = block_in_channels # additional dimension from skip connection being concatenated on
            block = nn.ModuleList()
            block_out_channels = block_in_channels // 2 
            for resnet_block in range(self.num_res_blocks):
                # print(f"up samp create block {resolution_level} {resnet_block} in channel {block_in_channels}+{skip_con_channels}={block_in_channels+skip_con_channels}-->{block_out_channels}")
                block.append(ResnetBlock(num_layers=self.num_res_layers,
                                         in_channels=block_in_channels + skip_con_channels,
                                         out_channels=block_out_channels,
                                         temb_channels=self.t_dim_expansion,
                                         dropout=self.dropout_percent, 
                                         kernel_size=self.kernel_sizes[self.kernel_counter], 
                                         activation_mode=self.activation_mode["upsampling"]))
                self.kernel_counter+=1
                # print("kernel:",self.kernel_counter,flush=True)
                skip_con_channels = 0 # only first block has input concatenated
                block_in_channels = block_out_channels
            up = nn.Module()
            if self.attn_resolutions[self.attn_counter]:
                up.attn = AttnBlock(block_in_channels)
            self.attn_counter+=1
            # print("attn:",self.attn_counter,flush=True)
            up.block = block
            extra_pad = 0 # no transform
            # print(f"resos({self.resos})[{resolution_level}]%2 == 1 : {self.resos[resolution_level]%2 == 1}",flush=True)
            if self.resos[resolution_level]%2 == 1: # odd//2 scales by 2x + 1 not just 2x
                extra_pad = 1 # increase output dimension by 1
            up.upsample = Upsample(block_in_channels, self.upsample_conv, extra_pad)
            # print("upsample block:",up.upsample,flush=True)
            self.up.insert(0, up)  # prepend to get consistent order

        #####################
        # End: output layer #
        #####################
        # print("output pass:",flush=True)
        self.norm_out = Normalize(block_in_channels)
        self.conv_out = torch.nn.Conv2d(block_in_channels,self.out_channels,kernel_size=self.kernel_sizes[self.kernel_counter],stride=1,padding=self.kernel_sizes[self.kernel_counter]//2)
        self.kernel_counter+=1
        # print("kernel:",self.kernel_counter,flush=True)


    def forward(self, x, t):
        x = x.to(device=t.device)

        # timestep embedding
        temb = get_time_embedding(t, self.t_emb_dim, self.embedding_mode).to(device=t.device)
        temb = self.temb.dense[0](temb)
        temb = activation(temb,self.activation_mode["input"])
        temb = self.temb.dense[1](temb)
        self.attn_counter = 0

        # downsampling
        h = self.conv_in(x)
        res_skips = []
        for resolution_level in range(self.num_resolutions): # looping over number of down sampling blocks
            # print(f"\n------------------------------\ndown samp reso {resolution_level}\n-----------------------------\n",flush=True)
            for resnet_block in range(self.num_res_blocks): # looping over res nets in each down sampling block
                # print(f"\n------------------------------\ndownsamp reso {resolution_level} resnet {resnet_block}\n-----------------------------\n",flush=True)
                # print(f"bef: {h.shape}",flush=True)
                h = self.down[resolution_level].block[resnet_block](h, temb) # forward pass on data
                # print(f"aft: {h.shape}",flush=True)
                # print(f"attn down {resolution_level} ({self.attn_resolutions})[{self.attn_counter}]")
                # print(f"from {self.down[resolution_level]}")
                if self.attn_resolutions[self.attn_counter]: # optional attention application
                    h = self.down[resolution_level].attn(h)
            self.attn_counter += 1
            # print(f"reso bef {resolution_level+1}/{self.num_resolutions} [{h.shape}] mode {self.downsample_mode}", flush=True)
            h = self.down[resolution_level].downsample(h) # reduce resolution by factor 2
            # print(f"reso aft {resolution_level+1}/{self.num_resolutions} [{h.shape}] mode {self.downsample_mode}", flush=True)
            if self.resolution_skips: # add skip connections to equal resolution in upsampling
                    # print(f"res_skip app: {h.shape}", flush=True)
                    res_skips.append(h)
        # print(f"\nresos ({len(res_skips)}):[{[r.shape for r in res_skips]}]\n",flush=True)

        # middle pass
        h = self.mid.block_1(h, temb)
        # print(f"attn bottom ({self.attn_resolutions})[{self.attn_counter}]")
        if self.attn_resolutions[self.attn_counter]:
            h = self.mid.attn(h)
        self.attn_counter += 1
        h = self.mid.block_2(h, temb)

        # upsampling
        for resolution_level in reversed(range(self.num_resolutions)): # up sampling iterates backwards through the resolutions
            # print(f"\n------------------------------\nupsamp reso {resolution_level}\n-----------------------------\n",flush=True)
            if self.resolution_skips: # optional skip connection
                # print(f"resskips {resolution_level}:{list(reversed(range(self.num_resolutions)))} cat {h.shape}, {(res_skips[resolution_level]).shape}",flush=True)
                # print(f"blocks:{self.up[resolution_level].block}",flush=True)
                # print(f"pre cat {h.shape}")
                h = torch.cat([h, res_skips[resolution_level]], dim=1)
                # print(f"post cat {h.shape}")
            for resnet_block in range(self.num_res_blocks): # pass through every res net block
                # print(f"\n------------------------------\nupsamp reso {resolution_level} resnet {resnet_block}\n-----------------------------\n",flush=True)
                # print(f"bef: {h.shape} pass into {self.up[resolution_level].block[resnet_block]}",flush=True)
                # print(f"upsample {resolution_level} resnet block {resnet_block}",flush=True)
                # print(f"pass in data {h.shape} in to {self.up[resolution_level].block[resnet_block]}")
                h = self.up[resolution_level].block[resnet_block](h, temb)
                # print(f"aft: {h.shape}",flush=True)
                # print(f"attn up {resolution_level} ({self.attn_resolutions})[{self.attn_counter}]")
                if self.attn_resolutions[self.attn_counter]: # optional attention mechanism
                    h = self.up[resolution_level].attn(h)
            self.attn_counter += 1
            # print(f"res {resolution_level} bef up {h.shape}, {self.up[resolution_level].upsample}",flush=True)
            # print(f"bef up: {h.shape}",flush=True)
            h = self.up[resolution_level].upsample(h)
            # print(f"aft up: {h.shape}",flush=True)
            # print(f"res {resolution_level} aft up {h.shape}",flush=True)


        # output
        h = self.norm_out(h)
        h = activation(h,self.activation_mode['output'])
        h = self.conv_out(h)
        return h


    @property
    def device(self) -> str:
        return next(self.parameters()).device
    
