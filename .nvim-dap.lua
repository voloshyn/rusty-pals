local dap = require("dap")

dap.adapters.lldb = {
    type = "executable",
    command = "/usr/bin/lldb-vscode",
    name = "lldb",
    initCommands = {"command script import ~/dev/intellij-rust/prettyPrinters/rust_types.py"}
}

dap.configurations.rust = {
    {
        name = "runner",
        type = "lldb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/runner"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
    }
}

