local dap = require("dap")

dap.adapters.lldb = {
    type = "executable",
    command = "/usr/bin/lldb-vscode",
    name = "lldb",
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
