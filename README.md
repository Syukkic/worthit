記錄自己買過咩東西

```bash
> worthit
記錄買過咩東西

Usage: worthit <COMMAND>

Commands:
  add     
  set     
  show    
  delete  

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

```bash
> worthit add --help
Usage: worthit add --name <NAME> --price <PRICE> --pd <PURCHASE_DATE>

Options:
  -n, --name <NAME>         佢個名
  -p, --price <PRICE>       幾钱
      --pd <PURCHASE_DATE>  幾時買，例如2024-1-6
  -h, --help                Print help
```

```bash
> worthit set --help
Usage: worthit set [OPTIONS]

Options:
  -n, --current-name <NAME>            佢個名
      --new-name <NEW_NAME>            起個新名
  -p, --price <PRICE>                  幾钱
      --purchase-date <PURCHASE_DATE>  幾時買，例如2024-1-6
  -s, --status <STATUS>                0: 用緊, 1: 食塵, 2: 壞咗, 3: 賣咗 [default: 0]
      --repair-count <REPAIR_COUNT>    整咗幾次
      --repair-cost <REPAIR_COST>      維修費
      --sold-price <SOLD_PRICE>        轉手價
      --sold-date <SOLD_DATE>          幾時賣，例如2025-1-6
  -h, --help                           Print help
```