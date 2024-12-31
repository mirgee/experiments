package main

import (
	"blockchain/blockchain"
	"fmt"
)

func main() {
	chain := blockchain.InitBlockchain()

	chain.AddBlock("Block 1")
	chain.AddBlock("Block 2")
	chain.AddBlock("Block 3")

	for i, block := range chain.Blocks {
		fmt.Printf("BLOCK %v\n", i+1)
		fmt.Printf("PrevHash: %x\n", block.PrevHash)
		fmt.Printf("Hash: %x\n", block.Hash)
		fmt.Printf("Data: %s\n", block.Data)
    pow := blockchain.NewProofOfWork(block)
		fmt.Printf("Is valid: %v\n", pow.Validate())
		fmt.Println()
	}
}
