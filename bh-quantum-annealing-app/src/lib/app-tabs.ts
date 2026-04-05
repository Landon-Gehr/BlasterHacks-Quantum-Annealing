export type TabId = "home" | "quantum" | "learned" | "live";

export type Tab = {
  id: TabId;
  label: string;
  caption: string;
};

export const tabs: Tab[] = [
  { id: "home", label: "Home", caption: "Landing" },
  { id: "quantum", label: "Quantum", caption: "Background" },
  { id: "learned", label: "Learned Solver", caption: "Model notes" },
  { id: "live", label: "Live Solver", caption: "Allen-Cahn demo" }
];
