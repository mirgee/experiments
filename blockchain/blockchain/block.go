package blockchain

import (
	"math/rand"
)

type Block struct {
	Hash         string
	Data         string
	PrevHash     string
	Nonce        int
	Transactions []*Transaction
}

// TODO: POW should not depend on Block
func CreateBlock(data string, prevHash string, txs []*Transaction) *Block {
	block := &Block{
		"",
		data,
		prevHash,
		rand.Intn(65536),
		txs,
	}
	pow := NewProofOfWork(block)
	nonce, hash := pow.MineBlock()
	block.Nonce = nonce
	block.Hash = string(hash[:])
	return block
}

func Genesis() *Block {
	return CreateBlock("genesis", "", []*Transaction{{
		"coinbase",
		"genesis",
		BlockReward,
		true,
	}})
}
