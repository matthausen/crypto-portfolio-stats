TODO


- use API to fetch current crypto value
https://pro-api.coinmarketcap.com

https://coinmarketcap.com/api/documentation/v1/#operation/getV1CryptocurrencyQuotesHistorical

- loop through the vector in the forntend and display


curl -H "X-CMC_PRO_API_KEY: db1fbebb-bd4d-4ffb-8f4f-1686b8771b84" -H "Accept: application/json" -G https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=BTC&convert=GBP


### Export and verify the file path of your csv file:

- `export CRYPTO=path/to/myfile.csv`

- `echo $CRYPTO`


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