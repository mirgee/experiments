package main

import (
	"blockchain/blockchain"
	"fmt"
)

func main() {
	chain := blockchain.InitBlockchain()

  aliceWallet, _ := blockchain.NewWallet()
  bobWallet, _ := blockchain.NewWallet()
  tx := &blockchain.Transaction{
  	Sender:   aliceWallet.PublicKey.N.String(),
  	Receiver: bobWallet.PublicKey.N.String(),
  	Amount:   5,
  	Coinbase: false,
  }
  sigBase64, err := aliceWallet.SignTransaction(tx)
  if err != nil {
    fmt.Println("Transaction signing failed:", err)
    return 
  }
  err = blockchain.VerifyTransaction(tx, aliceWallet.PublicKey, sigBase64)
  if err != nil {
    fmt.Println("Transaction signature invalid:", err)
    return
  }

	chain.AddBlock("Block 1", "Alice 1", []*blockchain.Transaction{tx})

	for i, block := range chain.Blocks {
		fmt.Printf("BLOCK %v\n", i+1)
		fmt.Printf("PrevHash: %x\n", block.PrevHash)
		fmt.Printf("Hash: %x\n", block.Hash)
		fmt.Printf("Data: %s\n", block.Data)
		pow := blockchain.NewProofOfWork(block)
		fmt.Printf("Is valid: %t\n", pow.Validate())

    for j, tx := range block.Transactions {
      fmt.Printf("TX %v\n", j+1)
      fmt.Printf("Sender: %s\n", tx.Sender)
      fmt.Printf("Receiver: %s\n", tx.Receiver)
      fmt.Printf("Amount: %f\n", tx.Amount)
      fmt.Printf("Coinbase: %t\n", tx.Coinbase)
    }
		fmt.Println()
	}
}
