import numpy as np
import matplotlib.pyplot as plt
import scipy.sparse as sp
from scipy.optimize import root
from matplotlib.animation import FuncAnimation

# Function to create 1D second-order finite difference matrix
def D2(n, delta):
    main_diagonal = -2 * np.ones(n)
    off_diagonal = np.ones(n - 1)
    return sp.diags([main_diagonal, off_diagonal, off_diagonal], [0, -1, 1]) / delta**2

# Function to create the 2D Laplacian using Kronecker product
def laplacian(D2_x, D2_y, n):
    I = sp.eye(n)
    return sp.kron(I, D2_x) + sp.kron(D2_y, I)

def residual_function(U_star, L, U_old, epsilon, delta_t):
    return ((U_star - U_old) / delta_t) - epsilon**2 * L.dot(U_star)

def integrate(U, delta_x, delta_y):
    return np.sum(U) * delta_x * delta_y

def F(U, X, Y):
    return U**3 - 2 * U

class Pixel_Draw:
    def __init__(self, image, brush_radius=3):
        self.image = image
        self.brush_radius = brush_radius
        self.drawn_mask = np.zeros(image.shape[:2], dtype=bool)
        self.drawing = False

        self.fig, self.ax = plt.subplots()
        self.ax.imshow(self.image, origin="upper", cmap="gray", interpolation="nearest")
        self.ax.set_title("Left click and drag to draw")

        self.overlay = self.ax.imshow(
            self.drawn_mask.astype(float),
            origin="upper",
            cmap="plasma",
            alpha=0.8,
            vmin=0,
            vmax=1,
            interpolation="nearest"
        )

        self.fig.canvas.mpl_connect("button_press_event", self.on_press)
        self.fig.canvas.mpl_connect("button_release_event", self.on_release)
        self.fig.canvas.mpl_connect("motion_notify_event", self.on_move)

    def paint_at(self, x, y):
        if x is None or y is None:
            return

        col = int(round(x))
        row = int(round(y))
        h, w = self.drawn_mask.shape

        for r in range(row - self.brush_radius, row + self.brush_radius + 1):
            for c in range(col - self.brush_radius, col + self.brush_radius + 1):
                if 0 <= r < h and 0 <= c < w:
                    if (r - row)**2 + (c - col)**2 <= self.brush_radius**2:
                        self.drawn_mask[r, c] = True

        self.update_overlay()

    def update_overlay(self):
        self.overlay.set_data(self.drawn_mask.astype(float))
        self.fig.canvas.draw_idle()

    def on_press(self, event):
        if event.inaxes != self.ax:
            return
        if event.button == 1:
            self.drawing = True
            self.paint_at(event.xdata, event.ydata)

    def on_release(self, event):
        if event.button == 1:
            self.drawing = False

    def on_move(self, event):
        if self.drawing and event.inaxes == self.ax:
            self.paint_at(event.xdata, event.ydata)

# -----------------------------
# Draw initial condition
# -----------------------------
N = 128
image = np.zeros((N, N))

drawer = Pixel_Draw(image, brush_radius=2)
plt.show()

drawn_pixels = np.argwhere(drawer.drawn_mask)
print("Mask shape:", drawer.drawn_mask.shape)
print("Number of drawn pixels:", drawer.drawn_mask.sum())
print("First 10 drawn pixel indices [row, col]:")
print(drawn_pixels[:10])

# -----------------------------
# Generate Grid, Initial Conditions, and Parameters
# -----------------------------
x = np.linspace(0, 5, N)
y = np.linspace(0, 5, N)
t = np.linspace(0, 50, 100)
I = np.zeros_like(t)

delta_x = x[1] - x[0]
delta_y = y[1] - y[0]
delta_t = t[1] - t[0]
epsilon = delta_x

X, Y = np.meshgrid(x, y)
noise_strength = 0.8

U = np.where(drawer.drawn_mask, 1.0, -1.0)
U = U + noise_strength * np.random.randn(N, N)
U = np.clip(U, -1.2, 1.2)
U0 = U.copy()

# Create Operator
D2_x = D2(N, delta_x)
D2_y = D2(N, delta_y)
L = laplacian(D2_x, D2_y, N)

# -----------------------------
# Solve and store frames
# -----------------------------
U_frames = [U.copy()]

for i in range(len(t)):
    U_old = U.flatten()

    residual = lambda U_star: residual_function(U_star, L, U_old, epsilon, delta_t)
    solution = root(residual, U_old, method='krylov', options={'fatol': 1e-8})
    U_star = solution.x.reshape((N, N))

    U = U_star - delta_t * F(U_star, X, Y)
    I[i] = integrate(U, delta_x, delta_y)

    U_frames.append(U.copy())

# -----------------------------
# Animate U
# -----------------------------
fig, ax = plt.subplots(figsize=(7, 6))

im = ax.imshow(
    U_frames[0],
    origin="lower",
    extent=[x.min(), x.max(), y.min(), y.max()],
    cmap="plasma",
    vmin=-1.5,
    vmax=1.5,
    interpolation="nearest"
)

cbar = fig.colorbar(im, ax=ax)
cbar.set_label("U values")

title = ax.set_title("Allen-Cahn Evolution, t = 0.00")
ax.set_xlabel("X")
ax.set_ylabel("Y")

def update(frame):
    im.set_data(U_frames[frame])
    time_value = frame * delta_t
    title.set_text(f"Allen-Cahn Evolution, t = {time_value:.2f}")
    return [im, title]

ani = FuncAnimation(
    fig,
    update,
    frames=len(U_frames),
    interval=100,
    blit=False,
    repeat=True
)

plt.tight_layout()
plt.show()