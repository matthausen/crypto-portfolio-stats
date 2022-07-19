Read from a csv list of positions opened on various trading platform.
Fetch the current price of those crypto assets from the CoinMarketPlace API and calculate the percentage of profit loss.

API Docs: https://coinmarketcap.com/api/documentation/v1/

### The CSV
Create a csv with this structure

```
date,asset,opened_amount,purchase_price
19-8-2021,BTC,100,34130.09
27-8-2021,ETH,80,35613.54
3-9-2021,ADA,50,36224
30-9-2021,SOL,30,2.1682
```

The asset is the symbol used by the CoinMarketPlace API to fetch the current price of the asset
The opened_amount is the the amount of fiat you used to buy the asset (by default everything is in GBP)
The purchase_price is the price in GBP of the asset when the purchase took place

### Export a couple of environment variable needed to run:

- `export CRYPTO=path/to/myfile.csv`
- `export API_KEY=<my-secret-api-key>`

### CoinMarketCap API Response example:

```
{
  "status": {
    "timestamp": "2021-10-31T09:47:43.953Z",
    "error_code": 0,
    "error_message": null,
    "elapsed": 19,
    "credit_count": 1,
    "notice": null
  },
  "data": {
    "BTC": {
      "id": 1,
      "name": "Bitcoin",
      "symbol": "BTC",
      "slug": "bitcoin",
      "num_market_pairs": 8479,
      "date_added": "2013-04-28T00:00:00.000Z",
      "tags": [
        "mineable",
        "pow",
        "sha-256",
        "store-of-value",
        "state-channels",
        "coinbase-ventures-portfolio",
        "three-arrows-capital-portfolio",
        "polychain-capital-portfolio",
        "binance-labs-portfolio",
        "arrington-xrp-capital",
        "blockchain-capital-portfolio",
        "boostvc-portfolio",
        "cms-holdings-portfolio",
        "dcg-portfolio",
        "dragonfly-capital-portfolio",
        "electric-capital-portfolio",
        "fabric-ventures-portfolio",
        "framework-ventures",
        "galaxy-digital-portfolio",
        "huobi-capital",
        "alameda-research-portfolio",
        "a16z-portfolio",
        "1confirmation-portfolio",
        "winklevoss-capital",
        "usv-portfolio",
        "placeholder-ventures-portfolio",
        "pantera-capital-portfolio",
        "multicoin-capital-portfolio",
        "paradigm-xzy-screener"
      ],
      "max_supply": 21000000,
      "circulating_supply": 18859643,
      "total_supply": 18859643,
      "is_active": 1,
      "platform": null,
      "cmc_rank": 1,
      "is_fiat": 0,
      "last_updated": "2021-10-31T09:47:02.000Z",
      "quote": {
        "GBP": {
          "price": 44414.43712459517,
          "volume_24h": 24206694877.516403,
          "volume_change_24h": -2.9721,
          "percent_change_1h": -0.21479832,
          "percent_change_24h": -1.63231998,
          "percent_change_7d": -2.54262812,
          "percent_change_30d": 35.30820549,
          "percent_change_60d": 27.74766993,
          "percent_change_90d": 53.88104287,
          "market_cap": 837640428215.8115,
          "market_cap_dominance": 44.0651,
          "fully_diluted_market_cap": 932703179616.4987,
          "last_updated": "2021-10-31T09:47:41.000Z"
        }
      }
    }
  }
}
```