package blockchain

import (
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
)

type Wallet struct {
	PrivateKey *rsa.PrivateKey
	PublicKey  *rsa.PublicKey
}

func GenerateRSAKeys() (*rsa.PrivateKey, *rsa.PublicKey, error) {
	privateKey, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return nil, nil, err
	}
	publicKey := &privateKey.PublicKey
	return privateKey, publicKey, nil
}

func NewWallet() (*Wallet, error) {
	PrivateKey, PublicKey, err := GenerateRSAKeys()
	if err != nil {
		return nil, err
	}
	return &Wallet{
		PrivateKey,
		PublicKey,
	}, nil
}

func (w *Wallet) SignTransaction(tx *Transaction) (string, error) {
	data := fmt.Sprintf("%s%s%f%t", tx.Sender, tx.Receiver, tx.Amount, tx.Coinbase)
	hash := sha256.Sum256([]byte(data))
  sigBytes, err := rsa.SignPKCS1v15(rand.Reader, w.PrivateKey, crypto.SHA256, hash[:])
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(sigBytes), nil
}

// TODO: Should return (bool, error)
func VerifyTransaction(tx *Transaction, publicKey *rsa.PublicKey, sigBase64 string) error {
	data := fmt.Sprintf("%s%s%f%t", tx.Sender, tx.Receiver, tx.Amount, tx.Coinbase)
	hash := sha256.Sum256([]byte(data))
  sigBytes, err := base64.StdEncoding.DecodeString(sigBase64)
  if err != nil {
    return err
  }
  return rsa.VerifyPKCS1v15(publicKey, crypto.SHA256, hash[:], sigBytes)
}
