export interface SorobanState {
  connected: boolean;
  rpcUrl: string;
  contractId: string;
}

export function useSoroban(): SorobanState {
  return {
    connected: false,
    rpcUrl: import.meta.env.VITE_RPC_URL ?? "",
    contractId: import.meta.env.VITE_CONTRACT_ID ?? "",
  };
}
