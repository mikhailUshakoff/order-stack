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
Import data from a file

* Usage: `order-stack import --folder-path <folder-path>`
* Options:
	+ `--folder-path` : Folder path containing data files

### `export`
Export data to a file

* Usage: `order-stack export --output-dir <output-dir>`
* Options:
	+ `--output-dir` : Output directory for exported data

### `summary`
Display a summary of the portfolio

* Usage: `order-stack summary`
