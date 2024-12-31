package blockchain

type Blockchain struct {
	Blocks []*Block
}

type Transaction struct {
	Sender   string
	Receiver string
	Amount   float64
	Coinbase bool
}

func InitBlockchain() *Blockchain {
	return &Blockchain{[]*Block{Genesis()}}
}

func (c *Blockchain) AddBlock(data string, coinbaseRcpt string, transactions []*Transaction) {
	lastBlock := c.Blocks[len(c.Blocks)-1]
	coinbaseTx := &Transaction{
		"coinbase",
		coinbaseRcpt,
		BlockReward,
		true,
	}
  transactions = append(transactions, coinbaseTx)
	newBlock := CreateBlock(data, lastBlock.Hash, transactions)
	c.Blocks = append(c.Blocks, newBlock)
}
