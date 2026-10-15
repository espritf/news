-- Rust
vim.lsp.config("rust_analyzer", {})
vim.lsp.enable("rust_analyzer")

-- SQL (sqls.nvim adds :SqlsExecuteQuery, :SqlsSwitchDatabase, ...)
vim.lsp.config("sqls", {
    settings = {
        sqls = {
            connections = {
                {
                    driver = "postgresql",
                    dataSourceName = os.getenv("DATABASE_URL"),
                },
            },
        },
    },
})
vim.lsp.enable("sqls")
