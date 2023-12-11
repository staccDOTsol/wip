import React from 'react';
import { ConnectionProvider, WalletProvider, useWallet } from '@solana/wallet-adapter-react';
import { WalletModalButton, WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import SolanaComponent from './SolanaComponent';

import '@solana/wallet-adapter-react-ui/styles.css';
import { connected } from 'process';
// You can also choose other network endpoints
const network = 'https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718';


function App() {
  const {connected} = useWallet();
  return (
    <ConnectionProvider endpoint={network}>
      <WalletProvider wallets={[]}>
      <WalletModalProvider>
        
          <div className="container">
            <header className="App-header">
              <h1>one-click megayield don't click this button button</h1> </header>
              <SolanaComponent />
          </div>
          
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export default App;
