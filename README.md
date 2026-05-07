# Order Stack

A command-line application for managing tokens portfolio.

## Table of Contents

* [Features](#features)
* [Commands](#commands)
	+ [add-token](#add-token)
	+ [remove-token](#remove-token)
	+ [list-tokens](#list-tokens)
	+ [add-order](#add-order)
	+ [list-orders](#list-orders)
	+ [remove-order](#remove-order)
	+ [import](#import)
	+ [export](#export)
	+ [summary](#summary)
* [Ratio calculation](#ratio-calculation)

## Features

* Token Portfolio overview
* Support for adding, removing, listing tokens and orders
* Support for importing and exporting data

## Commands

The following commands are available:

### `add-token`
Add a new token to the portfolio

* Usage: `order-stack add-token --name <name> --symbol <symbol>`
* Options:
	+ `--name` : Token name
	+ `--symbol` : Token symbol

### `remove-token`
Remove a token from the portfolio

* Usage: `order-stack remove-token --symbol <symbol>`
* Options:
	+ `--symbol` : Token symbol

### `list-tokens`
List all tokens in the portfolio

* Usage: `order-stack list-tokens`

### `add-order`
Add a new order to the portfolio

* Usage: `order-stack add-order --symbol <symbol> --side <side> --date <date> --volume <volume> --spent-usdt <spent-usdt> --note <note>`
* Options:
	+ `--symbol` : Token symbol
	+ `--side` : Order side (`buy` or `sell`)
	+ `--date` : Order date
	+ `--volume` : Order volume
	+ `--spent-usdt` : Order spent USDT
	+ `--note` : Order note (optional)

### `list-orders`
List all orders for a token

* Usage: `order-stack list-orders --symbol <symbol>`
* Options:
	+ `--symbol` : Token symbol

### `remove-order`
Remove an order from the portfolio

* Usage: `order-stack remove-order --id <id>`
* Options:
	+ `--id` : Order ID

### `import`
Import data from the `improt` folder

* Usage: `order-stack import`

### `export`
Export data to the `export` folder

* Usage: `order-stack export`

### `summary`
Display a summary of the portfolio

* Usage: `order-stack summary`

## Ratio calculation

For each token, the app first checks your net investment (`spent_usdt = buy_usdt - sell_usdt`).

* If net investment is still positive, it means you have put in more money than you have taken out, so ratio is calculated from current position value vs current net investment.
* If net investment is zero or negative, it means sells already covered all buys (or more), so ratio is calculated from total received value (`current_value + sell_usdt`) vs total buy cost (`buy_usdt`).

Formula reference:

* `RATIO` (per token):
	+ If `spent_usdt > 0`: `RATIO = (current_value / spent_usdt) * 100`
	+ Otherwise: `RATIO = ((current_value + sell_usdt) / buy_usdt) * 100`
* `TOTAL RATIO`: `TOTAL RATIO = (total_value / total_spent) * 100`
