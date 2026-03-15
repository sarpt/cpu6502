local dap = require('dap')

dap.adapters.codelldb = {
  type = "server",
  port = "${port}",
  executable = {
    command = 'codelldb',
    args = {"--port", "${port}"},
  },
}

dap.adapters.lldb = {
  type = 'executable',
  command = '/usr/bin/lldb-dap',
  name = 'lldb'
}

dap.configurations.cpp = {
  {
    name = 'Launch tests',
    type = 'codelldb',
    request = 'launch',
    program = function()
      return vim.fn.input('Path to test executable: ', vim.fn.getcwd() .. '/target/debug/deps/cpu6502-', 'file')
    end,
    cwd = '${workspaceFolder}',
    stopOnEntry = false,
    args = { },
    runInTerminal = false,
  },
}
dap.configurations.rust = dap.configurations.cpp
