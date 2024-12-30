package blockchain

import (
	"bytes"
	"crypto/md5"
)

type Block struct {
	Hash     string
	Data     string
	PrevHash string
}

func (b *Block) computeHash() {
	concatenated := bytes.Join([][]byte{[]byte(b.Data), []byte(b.PrevHash)}, []byte{})
	hash := md5.Sum(concatenated)
	b.Hash = string(hash[:])
}

func CreateBlock(data string, prevHash string) *Block {
	block := &Block{
		"",
		data,
		prevHash,
	}
  block.computeHash()
  return block
}

func Genesis() *Block {
  return CreateBlock("genesis", "")
}
