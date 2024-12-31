package main

import (
	"blockchain/blockchain"
	"fmt"
)

func main() {
	chain := blockchain.InitBlockchain()

	chain.AddBlock("Block 1", "Recipient 1", []*blockchain.Transaction{{
		Sender:   "Alice",
		Receiver: "Bob",
		Amount:   5,
	}, {
		Sender:   "Alice",
		Receiver: "Carol",
		Amount:   2,
	}})
	chain.AddBlock("Block 2", "Recipient 2", []*blockchain.Transaction{{
		Sender:   "Bob",
		Receiver: "Charlie",
		Amount:   1,
	}})
	chain.AddBlock("Block 3", "Recipient 3", []*blockchain.Transaction{{
		Sender:   "Charlie",
		Receiver: "Alice",
		Amount:   1,
	}})

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
