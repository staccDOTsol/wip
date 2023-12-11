import React from 'react';
import { ConnectionProvider, WalletProvider, useWallet } from '@solana/wallet-adapter-react';
import { WalletModalButton, WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import SolanaComponent from './SolanaComponent';

import '@solana/wallet-adapter-react-ui/styles.css';
import { connected } from 'process';
// You can also choose other network endpoints
const network = 'https://shallow-sharai-fast-mainnet.helius-rpc.com/';


function App() {
  const {connected} = useWallet();
  return (
    <ConnectionProvider endpoint={network}>
      <WalletProvider wallets={[]}>
      <WalletModalProvider>
        
          <div className="container">
            <header className="header">
              <h1>one-click 'megayield' don't click this button button</h1> </header>
              <SolanaComponent />
              <WalletMultiButton /> 
          </div>
          
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export default App;
