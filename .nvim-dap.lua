local dap = require("dap")

dap.adapters.lldb = {
    type = "executable",
    command = "/usr/bin/lldb-vscode", -- adjust as needed
    name = "lldb",
}

dap.configurations.rust = {
    {
        name = "clixed",
        type = "lldb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/clixed"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
    },
    {
        name = "hello-dap",
        type = "lldb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/clixe-dap"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
    },
}
