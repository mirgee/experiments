package blockchain

import (
	"bytes"
	"crypto/md5"
	"math/rand"
)

type Block struct {
	Hash         string
	Data         string
	PrevHash     string
	Nonce        int
	Transactions []*Transaction
}

func (b *Block) computeHash() {
	concatenated := bytes.Join([][]byte{[]byte(b.Data), []byte(b.PrevHash)}, []byte{})
	hash := md5.Sum(concatenated)
	b.Hash = string(hash[:])
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
