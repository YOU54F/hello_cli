import ctypes
import json

lib = ctypes.CDLL('target/release/libpact_cli_ffi.dylib')

lib.ffi_broker_list_latest_pact_versions.argtypes = [ctypes.c_char_p]
lib.ffi_broker_list_latest_pact_versions.restype = ctypes.c_char_p

lib.ffi_free_string.argtypes = [ctypes.c_char_p]

def generate_json(data):
    return json.dumps(data)

payload = {
    'broker_base_url': 'http://localhost:9292'
}
auth = {
    'broker_username': 'username',
    'broker_password': 'password'
}

command = 'some_command'
args = ['arg1', 'arg2']

payload.update(command=command, args=args, auth=auth)
cstring = generate_json(payload).encode('utf-8')
res = lib.ffi_broker_list_latest_pact_versions(cstring)
print(res.decode('utf-8'))

# Parse the result
result = json.loads(res.decode('utf-8'))
if result['code'] == 1:
    print(f"Code: {result['code']}")
    print(f"Reason: {result['reason']}")
    print(f"Message: {result['message']}")
else:
    print(f"Success: {result['message']}")
    print(f"Message: {result['message']}")

lib.ffi_free_string(res)

# Return exit code if the code is anything other than 0
if result['code'] != 0:
    exit(result['code'])
