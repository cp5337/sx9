from flask import Flask, request, jsonify
from flask_restx import Api, Resource, fields
import subprocess
import threading
import queue
import os

app = Flask(__name__)
api = Api(app, title='CTAS Kali Integration API', version='1.0')

# Command execution queue
cmd_queue = queue.Queue()

def run_command(command, output_queue):
    try:
        process = subprocess.Popen(
            command,
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            universal_newlines=True
        )
        stdout, stderr = process.communicate()
        output_queue.put({
            'stdout': stdout,
            'stderr': stderr,
            'returncode': process.returncode
        })
    except Exception as e:
        output_queue.put({
            'error': str(e),
            'returncode': -1
        })

@api.route('/tools/nmap')
class NmapScan(Resource):
    @api.doc(params={'target': 'Target IP or hostname', 'options': 'Scan options'})
    def post(self):
        data = request.json
        target = data.get('target')
        options = data.get('options', '-sV -sC')
        
        if not target:
            return {'error': 'Target is required'}, 400

        command = f'nmap {options} {target}'
        output_queue = queue.Queue()
        thread = threading.Thread(
            target=run_command,
            args=(command, output_queue)
        )
        thread.start()
        thread.join()
        
        result = output_queue.get()
        return result

@api.route('/tools/metasploit')
class Metasploit(Resource):
    @api.doc(params={'module': 'MSF module path', 'options': 'Module options'})
    def post(self):
        data = request.json
        module = data.get('module')
        options = data.get('options', {})
        
        if not module:
            return {'error': 'Module is required'}, 400

        # Build MSF resource script
        resource_script = f"""
use {module}
set RHOSTS {options.get('RHOSTS', '')}
set RPORT {options.get('RPORT', '')}
run
"""
        with open('/tmp/msf_resource', 'w') as f:
            f.write(resource_script)

        command = f'msfconsole -q -r /tmp/msf_resource'
        output_queue = queue.Queue()
        thread = threading.Thread(
            target=run_command,
            args=(command, output_queue)
        )
        thread.start()
        thread.join()
        
        result = output_queue.get()
        return result

@api.route('/tools/hydra')
class Hydra(Resource):
    @api.doc(params={'target': 'Target', 'service': 'Service type', 'wordlist': 'Path to wordlist'})
    def post(self):
        data = request.json
        target = data.get('target')
        service = data.get('service')
        wordlist = data.get('wordlist')
        
        if not all([target, service, wordlist]):
            return {'error': 'Target, service, and wordlist are required'}, 400

        command = f'hydra -L {wordlist} -P {wordlist} {target} {service}'
        output_queue = queue.Queue()
        thread = threading.Thread(
            target=run_command,
            args=(command, output_queue)
        )
        thread.start()
        thread.join()
        
        result = output_queue.get()
        return result

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)