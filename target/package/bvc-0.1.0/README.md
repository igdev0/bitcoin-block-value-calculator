# Block value calculator
This is a simple binary program which calculates the value of a given block on the Bitcoin network. You can specify the block by its height, hash, or get the best block.

# Setting up
You must set the following environment variables to setup the project.
`RPC_USERNAME=username`
`RPC_PASSWORD=password`
`RPC_DEFAULT_AUTH=true` // if set to false, you must set `RPC_USERNAME` & `RPC_PASSWORD`
`RPC_COOKIE_PATH=/Volumes/externalSSD/Bitcoin/.cookie` // This must be set if `RPC_DEFAULT_AUTH` is set to true 
`RPC_URL=http://localhost:8332`

# Building
`cargo run build`

# Instalation
`cargo install block-value-calculator`

# Usage
 `bvc -h 1000` // Calculates the total value of a block at height 1000

# Dev

`cargo run`


# Options

| Option   | Short | Description                                | Default |
| -------- | ----- | ------------------------------------------ | ------- |
| --height | -e    | Specify the block height                   |         |
| --hash   | -a    | Specify the block hash                     |         |
| --best   | -b    | Get the best block (set to true to enable) | true    |

# Environment variables