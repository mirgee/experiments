package blockchain

type Blockchain struct {
	Blocks []*Block
}

func InitBlockchain() *Blockchain {
	return &Blockchain{[]*Block{Genesis()}}
}

func (c *Blockchain) AddBlock(data string) {
  lastBlock := c.Blocks[len(c.Blocks) - 1]
  newBlock := CreateBlock(data, lastBlock.Hash)
  c.Blocks = append(c.Blocks, newBlock)
}
