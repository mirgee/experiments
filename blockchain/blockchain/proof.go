package blockchain

import (
	"bytes"
	"crypto/md5"
	"encoding/binary"
	"math/big"
)

const Difficulty = 10

type ProofOfWork struct {
	Block  *Block
	Target *big.Int
}

func NewProofOfWork(block *Block) *ProofOfWork {
	target := big.NewInt(1)
	target.Lsh(target, 256-Difficulty)
	return &ProofOfWork{
		block,
		target,
	}
}

func (pow *ProofOfWork) ComputeData(nonce int) []byte {
  buf := new(bytes.Buffer)
  buf.WriteString(pow.Block.PrevHash)
  buf.WriteString(pow.Block.Data);
  binary.Write(buf, binary.BigEndian, uint64(nonce));
  binary.Write(buf, binary.BigEndian, uint64(Difficulty));
  return buf.Bytes()
}

func (pow *ProofOfWork) MineBlock() (int, []byte) {
  for nonce := 0; ; nonce++ {
    data := pow.ComputeData(nonce)
    hash := md5.Sum(data)

    if new(big.Int).SetBytes(hash[:]).Cmp(pow.Target) == -1 {
      return nonce, hash[:]
    }
  }
}

func (pow *ProofOfWork) Validate() bool {
  data := pow.ComputeData(pow.Block.Nonce)
  hash := md5.Sum(data)

    if new(big.Int).SetBytes(hash[:]).Cmp(pow.Target) == -1 {
      return true
    } else {
      return false
    }
}
