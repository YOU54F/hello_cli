require 'fiddle'
require 'fiddle/import'

require 'json'
module PactCliFiddle
  extend Fiddle::Importer
  dlload File.expand_path('target/release/libpact_cli_ffi.dylib', Dir.pwd)

  extern 'const char *ffi_broker_list_latest_pact_versions(const char *args);'
  extern 'void ffi_free_string(char *ptr);'
end

def generate_json(data)
  JSON.generate(data)
end

payload = {
  broker_base_url: 'http://localhost:9292'
}
auth = {

  broker_username: 'username',
  broker_password: 'password'
}

command = 'some_command'
args = %w[arg1 arg2]

cstring = generate_json(payload.merge(command: command, args: args, auth: auth))
res = PactCliFiddle.ffi_broker_list_latest_pact_versions(cstring)
puts res

# Parse the result
result = JSON.parse(res.to_s)
if result['code'] == 1
  puts "Code: #{result['code']}"
  puts "Reason: #{result['reason']}"
  puts "Message: #{result['message']}"
else
  puts "Success: #{result['message']}"
  puts "Message: #{result['message']}"
end

PactCliFiddle.ffi_free_string(res)

# Return exit code if the code is anything other than 0
exit result['code'] unless result['code'] == 0
