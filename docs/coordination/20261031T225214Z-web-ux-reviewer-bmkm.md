---
kind: handoff
from: web-ux-reviewer
to: planner-architect,governor-reviewer
id: 20261031T225214Z-web-ux-reviewer-bmkm
timestamp: 2026-10-31T22:52:14Z
model: codex
immutable: true
references-to:
  - 20261031T224116Z-governor-reviewer-meph.md
  - 20261031T223613Z-governor-reviewer-g75t.md
  - 20261031T222335Z-governor-reviewer-h7gr.md
  - 20261031T222643Z-governor-reviewer-56qs.md
topic: a3-resource-v3b-executable-concurrence-packet
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the exact resource/lifecycle delta and labelled projections; explicitly concur or identify blockers for a separately authorized first-entry-only probe.
    - Preserve the narrow review scope and prior failures; this handoff grants no execution or acceptance authority.
  must_read:
    - docs/coordination/20261031T224116Z-governor-reviewer-meph.md
    - docs/coordination/20261031T222335Z-governor-reviewer-h7gr.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# A3 resource v3b: narrow executable concurrence requested

Claude: this appendix supplies six exact full production/control files and two explicitly labelled data projections. Verify each fenced payload's byte count and SHA-256, preserving the final newline. The projections are not runtime inputs and cannot replace the full ledger or recovery map. The complete private packet stays private; old artifact inventories and test source are not duplicated here.

Please review the resource and lifecycle changes only: first entry `telemetry-formulas-and-status-mobile-light` is the sole prospective launcher ID, with a 256 MiB whole-process cap, unchanged host MemAvailable admission of 320/300 MiB, zero swap, grouped OOM, 90 seconds, Node old-space 48 MiB and at most three JPEGs. The other thirteen ledger profiles retain 200 MiB and cannot be launched through this candidate launcher. This is not a nested-memory-limit change or general sizing claim.

The closed history is 13 images: six adopted, five failed and two diagnostic. Global remaining allowance is 43 under hard 56; unchanged local ceilings sum 46 and cannot override exhaustion. The h7gr case stays running/incomplete with no finished/close record, unit exit 1/oom-kill and zero credit. Neither the coordinate nor resource failure is erased. A prospective pass could establish only one entry's measured result once; it gives no later-entry, retry or cap-escalation authority.

The new helper takes one terminal systemctl snapshot and reads exact-cgroup counters if still available, before owned cleanup. Unavailable counters stay unavailable. Collection errors still reach cleanup and cannot turn failure into success. It is not a periodic sampler or per-process recorder and cannot promise allocation attribution. See the actual helper and launcher below.

Local checks: **285 = 264 substantive + 21 syntax**, all passed. They include actual Python-to-Node admission, first-ID refusal, host boundary checks, exact old failure/adoption hashes, 43-image exhaustion and mocked terminal-collection/cleanup failures. These are readiness tests, not fresh runtime evidence. Independent whole-ledger/history delta reviews are being assigned by root; this packet does not represent Claude as having read omitted files or rerun those tests.

The runner, measurement, clipping/character, native input, table and artifact inputs are byte-identical to reviewed v2; hashes below bind that claim. All 115 artifact files and ZIP were rehashed. Source is `6d47f8db63fa067bd633da13ede6ce76ef21a2dc`, build base `5a61048516d41f7aa70f9491ce3647867921d440`, accepted target at preparation `5b4c3f110347a1e5fafd15647becee07dafbe652`, pipeline 3012719521, HTML job 17635230933. No browser/service/probe has run in v3b.

Manual review still requires CSS1Compat in every saved snapshot; the unchanged validator also accepts BackCompat, so this is not claimed as an automatic rejection. Unsupported fixed/transform/contain/clip-path/mask chains and body-overflow behavior remain unproven. Independent actual-image review is required. No touch/keyboard/discoverability, full-site or R1/R2 clearance follows. Request explicit scoped executable concurrence for a future first-entry-only probe; exact independent reviews and a separate published root run decision remain required.

The original v3 controls and held kank draft remain historical and must not be used for this request. A narrow lifecycle review blocked v3 because loaded terminal properties were recorded without binding them. V3b now passes the first ledger profile to the collector and requires exact loaded Id/unit, a valid nonzero 32-hex InvocationID, owned absolute cgroup and matching MemoryMax/MemorySwapMax/OOMPolicy. Wrong or missing bindings fail even after a successful service. Any loaded nonterminal state or nonzero/missing MainPID triggers cleanup. Not-found units remain explicitly unavailable. This validates and records the terminal invocation identifier, not an invented pre-launch identifier. The 27 actual-function terminal tests include those negatives. Content/native/artifact/cap/budget remain unchanged.

## Frozen references; omitted files are not claimed reviewed by Claude

| File | Bytes | SHA-256 |
|---|---:|---|
| manifest.json | 9420 | b8f868746c3144aae37b09b58f7575af5033be85899b7af91bc9fea09999ecd3 |
| readiness.json | 5910 | 79e3133a80e231684cabb4b420b08afe880c98a03584668f0ebe4d644d92db11 |
| v3-to-v3b.patch | 20775 | ea388cbc49ad7ee8848451efcfd2c8e484b8a170e0d4a32a4f4190953d944aeb |
| groups.json | 7452 | 6baabfef08eea6fc599cc9931af31d50051d73ff449eb9f294df9fcd8bf96fa1 |
| recovery-map.json | 24282 | d3485a1ea84420905da2aaa3628e1ea2f4838e03f3309c4293c7dbe7c5acaf95 |
| run-once.cjs | 23583 | f0f7ff52726f6ebbc0b5d739ffd11439358cc4cb085fe56f6c4fc6dd678c3651 |
| code-measure.cjs | 6837 | 3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e |
| code-proof.cjs | 9405 | 2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df |
| capture-native.txt | 5624 | fa4632d5dca7b64e7ce521b0d367ba2f2d2a10021c2747576230b3011caca1b2 |
| scroll-evidence.cjs | 1500 | 548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b |
| table-guard.cjs | 4374 | 8caf8d55e33d99b210cc56cfe1a9101c08aa7f521e4ad9a44b3d76118257972f |
| inputs.json | 73079 | 51953b31b24b65731f09a647604236550ff6c985c8a2b932ea569394f10f683e |
| check-bindings.py | 6792 | f735c1c4a8c055247bd8eb5fd073d656a0886130c62119604704fc806e13a3be |
| check-code-offline.cjs | 7803 | b9a14e36fa52439ff5980dda9402bf5fa098b3a5bbb5479be59a06ad69e7d42f |
| check-content-offline.cjs | 3845 | 5820617a4fa5d20275d45a5076e969f0d6630c95ea288947b03ae0fe399a869e |
| check-coordinate-offline.cjs | 7594 | 62b913089373543de67f84c67f3aa4a1c60ffbb07f3762930a13748b34aada40 |
| check-handshake.py | 3522 | d3d9acce998a76a980d828979c059cc207bbbd07b6c11ac31acaa14c3ad2fc72 |
| check-offline.py | 6387 | c0266f88388962596ae64fa05c98a0e82764c0a8bb5feef469247bab0de18998 |
| check-recovery-offline.py | 5142 | 6ec7b7cec31905d2e0db9674563255680171d7c632e2faf095a7f053c35b0f70 |
| check-table-content-offline.cjs | 1521 | 4a2c9b906aef3bbed968683f17eb276360f996e228c2af55fc1aa839a8ca558c |
| check-table-offline.cjs | 4402 | 26a5752e1c08ad5601ac105929295890424e3b2a7f72ae3051c8df5301a8c2f0 |
| check-terminal-offline.py | 6800 | f996a2919e8de1ab1afebf57dbea73b50882e6fc16209c8d14033332431c938b |

Full candidate namespace: `/tmp/agf-a3-6d47f8db-resource-probe-v3b/`. The 48-file manifest excludes itself. This appendix contains no raw stderr, images, browser profile or private configuration.

## Reconstructable payloads

### launch-once.py

Exact full file. Bytes: 6097. SHA-256: `43115ed59a1ab981daa25fc20829b3ae29fa1e994f75ea0de50a1cbfa58ced22`.

<!-- packet-file: launch-once.py -->
````python
"""Run one phase only after root authorization; no retries or limit changes."""
import datetime, hashlib, json, pathlib, subprocess, sys, importlib.util
def expected_unit(ledger, group_id, script_root):
    if str(script_root) != ledger['execution']['namespaceRoot']:
        raise SystemExit('Execution namespace differs from frozen ledger')
    if not any(g['id']==group_id for g in ledger['groups']):
        raise SystemExit('Unknown execution group')
    return ledger['execution']['unitPrefix'] + group_id + '.service'

root = pathlib.Path(__file__).resolve().parent
ledger = json.loads((root / 'groups.json').read_text())
group_id=sys.argv[1] if len(sys.argv)==2 else ''
groups=ledger['groups']
group=next((g for g in groups if g['id']==group_id),None)
if group is None:raise SystemExit('Exactly one fixed reviewed A3 group required')
if group_id != groups[0]['id']:raise SystemExit('Resource probe permits first entry only')
case=group_id
unit = expected_unit(ledger, group_id, root)
expected = {'run-once.cjs': 'f0f7ff52726f6ebbc0b5d739ffd11439358cc4cb085fe56f6c4fc6dd678c3651', 'admission-guard.cjs': 'd7658c2a69afb11e49b0dff3f41a52d9c5e9ef8f039d03ab7f977f07b66dc5ca', 'table-guard.cjs': '8caf8d55e33d99b210cc56cfe1a9101c08aa7f521e4ad9a44b3d76118257972f', 'code-measure.cjs': '3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e', 'code-proof.cjs': '2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df', 'scroll-evidence.cjs': '548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b', 'recovery-guard.py': '2082846e5de470e39142242295eacefe8f5aaaa6eeb0b0abf1064b475e6844df', 'recovery-map.json': 'd3485a1ea84420905da2aaa3628e1ea2f4838e03f3309c4293c7dbe7c5acaf95', 'inputs.json': '51953b31b24b65731f09a647604236550ff6c985c8a2b932ea569394f10f683e', 'spec.json': 'df1d0216635918b8a1250ac25f37b1ac62da0c9e76e2d42f29a9f47f893ef896', 'groups.json': '6baabfef08eea6fc599cc9931af31d50051d73ff449eb9f294df9fcd8bf96fa1', 'artifact_parser.py': 'a730f684bea9887d1c7062b66eaa9df4058cccd8e1c7ce15b26f468f6acc0998', 'job.json': '30b79394cec7cf84fe3210232e64909d42acf03fccace9368ddfccd841e4a895', 'meph-policy-sidecar.json': '22c2d4d166ef2922f4b873e262c525161f1a55b7f060d6978f88e238582b57e3', 'terminal-evidence.py': 'eb6c9aebed4dbab1f53f34c359dc0f2738b63d071424115a91d1c2ed5a2497d3'}
for name, sha in expected.items():
    if hashlib.sha256((root / name).read_bytes()).hexdigest() != sha:
        raise SystemExit('Reviewed input changed: ' + name)
# Exact closed external adoption/failure map; no generic skip-failed option.
module_spec=importlib.util.spec_from_file_location('recovery_guard',root/'recovery-guard.py');recovery_guard=importlib.util.module_from_spec(module_spec);module_spec.loader.exec_module(recovery_guard)
recovery_guard.validate_recovery(root,ledger)
# Reject every incomplete or failed predecessor; no adopted/recovery exceptions.
for prior in groups[:groups.index(group)]:
    result_path=root / ('case-'+prior['id']) / 'results.json'
    cleanup_path=root / (prior['id']+'-cleanup.json')
    if not result_path.exists() or not cleanup_path.exists():raise SystemExit('Earlier group absent/incomplete; stop sequence')
    prior_result=json.loads(result_path.read_text()); cleanup=json.loads(cleanup_path.read_text())
    runs=prior_result.get('runs',[])
    if not prior_result.get('finished') or prior_result.get('failure') or len(runs)!=1 or not runs[0].get('afterBrowser') or cleanup.get('systemdRunExit')!=0 or cleanup.get('remainingOwnedProcesses'):
        raise SystemExit('Earlier group failed/incomplete; no continuation')
    cases=runs[0].get('cases',[])
    if [c['name'] for c in cases]!=prior['expectedCases'] or any(c.get('status')!='completed' for c in cases):raise SystemExit('Earlier group coverage incomplete')
marker = root / (case + '-parent-guard.json')
attempt_paths = [marker, root / ('case-' + case), root / (case + '-service.log'), root / (case + '-cleanup.json')]
if any(path.exists() or path.is_symlink() for path in attempt_paths):
    raise SystemExit('Phase already attempted; no retry')
mem = dict(line.split(':', 1) for line in pathlib.Path('/proc/meminfo').read_text().splitlines())
available = int(mem['MemAvailable'].split()[0])
if available < group['outerKiB']:
    raise SystemExit('Outer group memory guard failed; no attempt marker or browser')
outer_available = available
mem = dict(line.split(':',1) for line in pathlib.Path('/proc/meminfo').read_text().splitlines())
available = int(mem['MemAvailable'].split()[0])
guard = {'outerAvailableKiB': outer_available, 'outerMinimumKiB': group['outerKiB'], 'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'memAvailableKiB': available, 'minimumKiB': group['innerKiB'], 'inputSha256': expected}
marker.write_text(json.dumps(guard, indent=2) + '\n')
if available < group['innerKiB']:
    raise SystemExit('Memory guard failed; browser not launched')
cmd = ['systemd-run', '--user', '--unit=' + unit, '--service-type=exec', '--wait', '--pipe', '-p', 'MemoryMax='+str(group['memoryMaxBytes']), '-p', 'MemorySwapMax=0', '-p', 'OOMPolicy=kill', '-p', 'KillMode=control-group', '-p', 'RuntimeMaxSec=90s', '/usr/bin/env', 'PLAYWRIGHT_BROWSERS_PATH=/home/dev/s/tmp/agf-browser-baseline', 'DEBUG=pw:browser', 'node', '--max-old-space-size=48', str(root / 'run-once.cjs'), group_id]
with (root / (case + '-service.log')).open('x') as log:
    result = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT)
# One host-side terminal snapshot before exact owned-unit cleanup; no recorder.
terminal_spec=importlib.util.spec_from_file_location('terminal_evidence',root/'terminal-evidence.py');terminal=importlib.util.module_from_spec(terminal_spec);terminal_spec.loader.exec_module(terminal)
cleanup=terminal.finish_unit(unit,result.returncode, {**group, 'memorySwapMax': 0, 'oomPolicy': 'kill'})
(root / (case + '-cleanup.json')).write_text(json.dumps(cleanup, indent=2) + '\n')
print(json.dumps(cleanup))
raise SystemExit(result.returncode or (1 if cleanup['remainingOwnedProcesses'] or cleanup['collectionErrors'] or cleanup['cleanupErrors'] else 0))
````

### terminal-evidence.py

Exact full file. Bytes: 5595. SHA-256: `eb6c9aebed4dbab1f53f34c359dc0f2738b63d071424115a91d1c2ed5a2497d3`.

<!-- packet-file: terminal-evidence.py -->
````python
"""One terminal snapshot, then exact owned-unit cleanup; never a sampler."""
import datetime
import pathlib
import subprocess
import re

PROPERTIES = ['Id', 'InvocationID', 'ControlGroup', 'MemoryMax', 'MemorySwapMax',
              'OOMPolicy', 'MemoryCurrent', 'MemoryPeak', 'MemorySwapPeak',
              'Result', 'ExecMainCode', 'ExecMainStatus', 'LoadState',
              'ActiveState', 'SubState', 'MainPID']
COUNTERS = ['memory.current', 'memory.peak', 'memory.events', 'memory.events.local',
            'memory.max', 'memory.swap.current', 'memory.swap.peak',
            'memory.swap.max', 'memory.oom.group']

def finish_unit(unit, run_exit, expected_profile):
    if not unit.endswith('.service') or '/' in unit or any(c.isspace() for c in unit):
        raise ValueError('Not the exact owned probe unit')
    out = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
           'unit': unit, 'systemdRunExit': run_exit, 'expectedProfile': expected_profile,
           'bindingStatus': 'unavailable', 'collectionErrors': [],
           'cleanupErrors': [], 'remainingOwnedProcesses': [],
           'properties': {name: {'value': None, 'available': False} for name in PROPERTIES}, 'cgroupCounters': {}}
    props = {}
    try:
        args = ['systemctl', '--user', 'show', unit]
        for name in PROPERTIES:
            args += ['-p', name]
        state = subprocess.run(args, capture_output=True, text=True, timeout=10)
        out['terminalStateBeforeCleanup'] = state.stdout
        out['showExit'] = state.returncode
        out['showStderr'] = state.stderr
        props = dict(line.split('=', 1) for line in state.stdout.splitlines() if '=' in line)
        for name in PROPERTIES:
            value = props.get(name)
            out['properties'][name] = {'value': value, 'available': value not in (None, '', '[not set]')}
        # Not-found evidence is unavailable; a still-loaded unit must match all bindings.
        if props.get('LoadState') == 'not-found':
            out['cgroupUnavailable'] = 'Unit already not found; no terminal binding or counters inferred'
            return out
        if state.returncode or props.get('LoadState') != 'loaded':
            raise RuntimeError('Terminal unit loaded-state snapshot unavailable or invalid')
        if props.get('Id') != unit:
            raise RuntimeError('Terminal unit identity mismatch')
        invocation = props.get('InvocationID', '')
        if not re.fullmatch(r'[0-9a-f]{32}', invocation) or invocation == '0'*32:
            raise RuntimeError('Terminal invocation identifier missing or invalid')
        expected_limits = {'MemoryMax': str(expected_profile['memoryMaxBytes']),
                           'MemorySwapMax': str(expected_profile['memorySwapMax']),
                           'OOMPolicy': expected_profile['oomPolicy']}
        if any(props.get(name) != value for name, value in expected_limits.items()):
            raise RuntimeError('Terminal configured limits differ from frozen profile')
        cg = props.get('ControlGroup', '')
        parts = pathlib.PurePosixPath(cg).parts
        if not cg.startswith('/') or '..' in parts or not parts or parts[-1] != unit:
            raise RuntimeError('Terminal cgroup identity missing or mismatched')
        out['bindingStatus'] = 'bound'
        if cg:
            base = pathlib.Path('/sys/fs/cgroup' + cg)
            for name in COUNTERS:
                try:
                    out['cgroupCounters'][name] = {'available': True, 'value': (base / name).read_text().strip()}
                except OSError as error:
                    out['cgroupCounters'][name] = {'available': False, 'reason': type(error).__name__}
        else:
            out['cgroupUnavailable'] = 'No terminal ControlGroup property; no counters inferred'
    except Exception as error:
        out['bindingStatus'] = 'invalid'
        out['collectionErrors'].append(type(error).__name__ + ': ' + str(error))
    finally:
        # Collection failure must not prevent cleanup of the known owned unit.
        if out['collectionErrors'] or (props.get('LoadState') == 'loaded' and (props.get('ActiveState') != 'inactive' or props.get('SubState') != 'dead' or props.get('MainPID') != '0')):
            for action in ('stop', 'reset-failed'):
                try:
                    result = subprocess.run(['systemctl', '--user', action, unit], capture_output=True, text=True, timeout=10)
                    if result.returncode:
                        out['cleanupErrors'].append({'action': action, 'exit': result.returncode, 'stderr': result.stderr})
                except Exception as error:
                    out['cleanupErrors'].append({'action': action, 'error': type(error).__name__ + ': ' + str(error)})
        try:
            for proc in pathlib.Path('/proc').iterdir():
                if not proc.name.isdigit():
                    continue
                try:
                    cg = (proc / 'cgroup').read_text()
                    if any(line.partition('::')[2].strip().endswith('/' + unit) or '/'+unit+'/' in line for line in cg.splitlines()):
                        out['remainingOwnedProcesses'].append({'pid': int(proc.name), 'cgroup': cg.strip()})
                except (FileNotFoundError, ProcessLookupError):
                    pass
                except OSError as error:
                    out['cleanupErrors'].append({'pid': proc.name, 'error': type(error).__name__})
        except OSError as error:
            out['cleanupErrors'].append({'processInventory': type(error).__name__})
    return out
````

### recovery-guard.py

Exact full file. Bytes: 6330. SHA-256: `2082846e5de470e39142242295eacefe8f5aaaa6eeb0b0abf1064b475e6844df`.

<!-- packet-file: recovery-guard.py -->
````python
from pathlib import Path
import json,hashlib

def read_json(f):return json.loads(Path(f).read_text())
def hashed(record):
 p=Path(record['file'])
 if not p.is_file() or p.is_symlink() or hashlib.sha256(p.read_bytes()).hexdigest()!=record['sha256']:raise SystemExit('Closed recovery evidence changed/missing')
 return p

def check_original_records(m,records,ledger):
 if len(m['originalGroups'])!=7 or len(records)!=7 or len(ledger['groups'])!=14:raise SystemExit('Closed recovery cardinality')
 if [r['originalLogicalGroup']for r in m['originalGroups']]!=list(range(1,8)):raise SystemExit('Original order')
 if [r['credit']for r in m['originalGroups']]!=[1]*6+[0]:raise SystemExit('Failed group must retain zero credit')
 for index,(binding,pair)in enumerate(zip(m['originalGroups'],records)):
  result,cleanup=pair;cases=[c for run in result.get('runs',[])for c in run.get('cases',[])]
  if result['groupId']!=binding['id'] or len(cases)!=1 or cleanup.get('remainingOwnedProcesses'):raise SystemExit('Original identity/cleanup changed')
  if index<6:
   if binding['expectedStatus']!='completed' or cases[0]['status']!='completed' or result.get('failure') or not result.get('finished') or cleanup['systemdRunExit']!=0:raise SystemExit('Adopted original incomplete/failed')
  elif binding['expectedStatus']!='failed' or cases[0]['status']!='failed' or not result.get('failure') or cleanup['systemdRunExit']!=1:raise SystemExit('Original failed attempt relabelled')
 if [g['id']for g in ledger['groups']]!=[m['originalGroups'][6]['id']]+[g['id']for g in m['originalUnattempted']]:raise SystemExit('Closed 14-entry map changed')
 if m['acceptedImages']!=6 or m['failedImages']!=5 or m['diagnosticImages']!=2 or m['diagnosticMatrixCredit']!=0:raise SystemExit('Image/credit accounting')
 if sum(g['frameBudget']for g in ledger['groups'])!=46 or ledger['externalPhysicalImages']!=13 or ledger['newPhysicalCeiling']!=43 or ledger['allHistoryPhysicalMaximum']!=56 or ledger['hardPhysicalCeiling']!=56:raise SystemExit('Physical ceiling changed')
 return True

def check_failed_recovery(m,result,cleanup,ledger):
 b=m['recoveryFailedAttempt'];cases=[c for r in result.get('runs',[])for c in r.get('cases',[])]
 if b['id']!=ledger['groups'][0]['id'] or b['credit']!=0 or b['expectedStatus']!='failed' or b['expectedImages']!=2 or result['groupId']!=b['id']:raise SystemExit('Failed recovery identity/credit changed')
 if len(cases)!=1 or cases[0]['status']!='failed' or not result.get('failure') or cases[0].get('codeGestures')!=[] or len(cases[0].get('screenshots',[]))!=2 or cleanup['systemdRunExit']!=1 or cleanup.get('remainingOwnedProcesses'):raise SystemExit('Failed recovery relabelled/incomplete history')
 if b['unattemptedIds']!=[g['id']for g in ledger['groups'][1:]] or m['historicalTotalImages']!=13:raise SystemExit('Failed recovery continuation/history map changed')
 return True

def check_resource_failure(m,result,cleanup,ledger):
 b=m['resourceFailedAttempt'];runs=result.get('runs',[]);cases=[c for r in runs for c in r.get('cases',[])]
 if b['id']!=ledger['groups'][0]['id'] or result.get('groupId')!=b['id'] or b['credit']!=0 or b['expectedStatus']!='running' or b['expectedImages']!=2:raise SystemExit('Resource failure identity/credit changed')
 if len(runs)!=1 or len(cases)!=1 or cases[0].get('status')!='running' or result.get('finished') or result.get('failure') or runs[0].get('afterBrowser') or any('close' in k.lower() for k in runs[0]):raise SystemExit('Resource failure completion fabricated')
 if len(cases[0].get('screenshots',[]))!=2 or cleanup.get('systemdRunExit')!=1 or cleanup.get('remainingOwnedProcesses'):raise SystemExit('Resource failure image/cleanup changed')
 props=dict(line.split('=',1) for line in cleanup.get('terminalStateBeforeCleanup','').splitlines() if '=' in line)
 if props.get('Result')!='oom-kill' or props.get('ExecMainStatus')!='9':raise SystemExit('Resource OOM terminal identity changed')
 if b['unattemptedIds']!=[g['id'] for g in ledger['groups'][1:]] or m['historicalTotalImages']!=13:raise SystemExit('Resource failure continuation/history changed')
 return True

def validate_recovery(root,ledger):
 m=read_json(root/'recovery-map.json');records=[]
 for binding in m['originalGroups']:
  paths=[hashed(f)for f in binding['files']]
  result=next(p for p in paths if p.name=='results.json');cleanup=next(p for p in paths if p.name.endswith('-cleanup.json'))
  if len([p for p in paths if p.suffix=='.jpg'])!=1:raise SystemExit('Expected exactly one original image')
  records.append((read_json(result),read_json(cleanup)))
 for f in m['diagnosticFiles']+m['historicalManifests']+m['adoptionReviews']:hashed(f)
 check_original_records(m,records,ledger)
 failed=m['recoveryFailedAttempt'];paths=[hashed(f)for f in failed['files']]
 if len([f for f in paths if f.suffix=='.jpg'])!=2:raise SystemExit('Failed recovery image inventory changed')
 check_failed_recovery(m,read_json(next(f for f in paths if f.name=='results.json')),read_json(next(f for f in paths if f.name.endswith('-cleanup.json'))),ledger)
 for gid in failed['unattemptedIds']:
  prior=Path(failed['root']);paths=[prior/('case-'+gid)]+[prior/(gid+s)for s in ['-parent-guard.json','-service.log','-cleanup.json']]
  if any(f.exists()or f.is_symlink()for f in paths):raise SystemExit('Prior failed namespace has unexpected later attempt')
 for g in m['originalUnattempted']:
  old=Path(m['originalRoot']);paths=[old/('case-'+g['id'])]+[old/(g['id']+s)for s in ['-parent-guard.json','-service.log','-cleanup.json']]
  if any(p.exists()or p.is_symlink()for p in paths):raise SystemExit('Original unattempted group acquired output')
 resource=m['resourceFailedAttempt'];paths=[hashed(f) for f in resource['files']]
 if len([f for f in paths if f.suffix=='.jpg'])!=2:raise SystemExit('Resource failed image inventory changed')
 check_resource_failure(m,read_json(next(f for f in paths if f.name=='results.json')),read_json(next(f for f in paths if f.name.endswith('-cleanup.json'))),ledger)
 for gid in resource['unattemptedIds']:
  prior=Path(resource['root']);attempts=[prior/('case-'+gid)]+[prior/(gid+s) for s in ['-parent-guard.json','-service.log','-cleanup.json']]
  if any(f.exists() or f.is_symlink() for f in attempts):raise SystemExit('Resource failed namespace has unexpected later attempt')
 return m
````

### admission-guard.cjs

Exact full file. Bytes: 685. SHA-256: `d7658c2a69afb11e49b0dff3f41a52d9c5e9ef8f039d03ab7f977f07b66dc5ca`.

<!-- packet-file: admission-guard.cjs -->
````javascript
'use strict';
function checkAdmission(ledger,groupId,scriptRoot,proof){
  const batch=ledger.groups.find(g=>g.id===groupId);
  if(!batch || scriptRoot!==ledger.execution.namespaceRoot)throw Error('Execution namespace/group differs from frozen ledger');
  const unit=ledger.execution.unitPrefix+groupId+'.service';
  if(!proof.cgroup.endsWith('/'+unit)||proof.max==='max'||Number(proof.max)<=0||Number(proof.max)!==batch.memoryMaxBytes||proof.swap!=='0'||proof.oomGroup!=='1')throw Error('Isolation guard failed before browser-library load');
  return {unit,namespace:scriptRoot,capBytes:batch.memoryMaxBytes,swap:proof.swap,oomGroup:proof.oomGroup};
}
module.exports={checkAdmission};
````

### spec.json

Exact full file. Bytes: 1527. SHA-256: `df1d0216635918b8a1250ac25f37b1ac62da0c9e76e2d42f29a9f47f893ef896`.

<!-- packet-file: spec.json -->
````json
{
  "status": "meph resource-only first-entry preparation; no runtime authority",
  "source": "6d47f8db63fa067bd633da13ede6ce76ef21a2dc",
  "method": "Exact native CODE per-line union and whole-character visibility, with measured scroll owner, clipping ancestors and visible controls; no PRE-fit exemption",
  "groups": 14,
  "priorAdopted": 6,
  "failedOldGroup7Credit": 0,
  "frames": {
    "remainingGlobalMaximum": 43,
    "sumLocalCeilings": 46,
    "retainedAccepted": 6,
    "retainedFailed": 5,
    "diagnostic": 2,
    "allPhysicalMaximum": 56,
    "originalGlobalCeiling": 56
  },
  "gestures": {
    "perEligibleCode": 1,
    "wheelDelta": [
      512,
      0
    ],
    "maximumAcrossLedger": 10
  },
  "limits": [
    "Historical six observations retain original image/aggregate evidence; no invented character telemetry",
    "No touch/keyboard discoverability or full-site/R1/R2 approval",
    "No source/style changes or current acceptance waiver"
  ],
  "inputArtifactBase": "5a61048516d41f7aa70f9491ce3647867921d440",
  "currentTarget": "5b4c3f110347a1e5fafd15647becee07dafbe652",
  "scopeDecision": "20261031T224116Z-governor-reviewer-meph",
  "coordinateContract": "Actual document.scrollingElement identity owns viewport clipping; ordinary nested padding-box clips and all occlusion remain.",
  "firstEntryProfile": {
    "memoryMaxBytes": 268435456,
    "outerKiB": 327680,
    "innerKiB": 307200,
    "hostThresholdsNotNestedCaps": true,
    "maximumImages": 3,
    "otherProfilesUnchanged": true
  }
}
````

### meph-policy-sidecar.json

Exact full file. Bytes: 1332. SHA-256: `22c2d4d166ef2922f4b873e262c525161f1a55b7f060d6978f88e238582b57e3`.

<!-- packet-file: meph-policy-sidecar.json -->
````json
{
  "status": "meph preparation only; no run or acceptance authority",
  "decision": "20261031T224116Z-governor-reviewer-meph at c8c5785f",
  "currentBudget": {
    "historicalImages": 13,
    "adopted": 6,
    "failed": 5,
    "diagnostic": 2,
    "globalRemaining": 43,
    "sumLocalCeilings": 46,
    "hardOverall": 56
  },
  "onlyProspectiveEntry": "telemetry-formulas-and-status-mobile-light",
  "capBytes": 268435456,
  "hostAdmissionKiB": [
    327680,
    307200
  ],
  "conditions": [
    "Only first entry may be launched after separate published authority; other thirteen profiles remain 200 MiB.",
    "All saved snapshots require manual CSS1Compat-only review; validator itself still allows BackCompat.",
    "Maximum three images for first entry; all physical images count, including failures and diagnostic.",
    "No general fixed/transform/contain/clip-path/mask rejection or body-overflow proof; independent images required.",
    "No automatic retries, cap escalation, later groups, source edits or upload.",
    "One-shot host terminal evidence before owned cleanup, unavailable counters explicit; no sampler."
  ],
  "historicalPolicy": {
    "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/6hma-policy-sidecar.json",
    "sha256": "98434b92fca1013d315209f03ee588c0201b74f25489d4a0714d7010ac0a589e"
  }
}
````

### first-ledger-entry-projection.json

Labelled projection only; source hash and extraction rule are inside. Bytes: 1457. SHA-256: `198303e950404071195830a9a6573b6be60024e95b9eb89c2f3de56384be51e7`.

<!-- packet-file: first-ledger-entry-projection.json -->
````json
{
  "projectionOnly": true,
  "sourceFile": "groups.json",
  "sourceSha256": "6baabfef08eea6fc599cc9931af31d50051d73ff449eb9f294df9fcd8bf96fa1",
  "projectionRule": "Copy exactly the listed top-level fields and groups[0]; omit configurations and the thirteen other groups. This is not a replacement runtime ledger.",
  "selectedTopLevelFields": {
    "status": "meph preparation only; first entry resource probe only; no run authority",
    "source": "6d47f8db63fa067bd633da13ede6ce76ef21a2dc",
    "executionAuthorized": false,
    "execution": {
      "namespaceRoot": "/tmp/agf-a3-6d47f8db-resource-probe-v3b",
      "unitPrefix": "agf-a3-resource-v3b-"
    },
    "externalPhysicalImages": 13,
    "newPhysicalCeiling": 43,
    "allHistoryPhysicalMaximum": 56,
    "originalLogicalStart": 7,
    "hardPhysicalCeiling": 56,
    "localCeilingsSum": 46,
    "budgetPolicy": "Global 43 remaining overrides sum 46 local ceilings; all 13 historical images charged; first entry only."
  },
  "firstGroup": {
    "id": "telemetry-formulas-and-status-mobile-light",
    "surface": "telemetry-method-and-limits",
    "configuration": "mobile-light",
    "phase": "telemetry-formulas-and-status",
    "targetIds": [
      "telemetry-formulas-and-status"
    ],
    "expectedCases": [
      "telemetry-formulas-and-status"
    ],
    "frameBudget": 3,
    "memoryMaxBytes": 268435456,
    "outerKiB": 327680,
    "innerKiB": 307200
  },
  "omittedGroupCount": 13
}
````

### h7gr-failure-closure-projection.json

Labelled projection only; source hash and extraction rule are inside. Bytes: 3728. SHA-256: `b17d7450e3f4a4568a4566577f4dd0b137aaa3f62704ce8370dc059a819d3459`.

<!-- packet-file: h7gr-failure-closure-projection.json -->
````json
{
  "projectionOnly": true,
  "sourceFile": "recovery-map.json",
  "sourceSha256": "d3485a1ea84420905da2aaa3628e1ea2f4838e03f3309c4293c7dbe7c5acaf95",
  "projectionRule": "Copy exactly resourceFailedAttempt and selected historical totals; omit earlier originals, qg8d failure and adoption bindings. Those remain hash-bound locally, not reproduced.",
  "historicalTotals": {
    "acceptedImages": 6,
    "failedImages": 5,
    "diagnosticImages": 2,
    "diagnosticMatrixCredit": 0,
    "historicalTotalImages": 13
  },
  "resourceFailedAttempt": {
    "root": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2",
    "id": "telemetry-formulas-and-status-mobile-light",
    "credit": 0,
    "expectedStatus": "running",
    "expectedImages": 2,
    "reason": "h7gr service OOM; no terminal case completion",
    "files": [
      {
        "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/case-telemetry-formulas-and-status-mobile-light/results.json",
        "sha256": "c13d46383dc515817be14cf009f7767e840fc5bc6d4d2e9bc6c93372da9fc61c"
      },
      {
        "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/case-telemetry-formulas-and-status-mobile-light/child-limit-proof.json",
        "sha256": "a5e70725aae72bfeac58d200750a509aebe8cfbf657918718360d28de37e8eee"
      },
      {
        "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/case-telemetry-formulas-and-status-mobile-light/mobile-light-telemetry-formulas-and-status-0-code-p26-native-end.jpg",
        "sha256": "681050fbaf2926af9c4cf0469c66d1a0e635fe63049beb8f9310c74a6bc888aa"
      },
      {
        "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/case-telemetry-formulas-and-status-mobile-light/mobile-light-telemetry-formulas-and-status-0.jpg",
        "sha256": "77c9d9da3592fa4cadc76595de7157da70ff66afac6f1550fe489cdc38e2f15e"
      },
      {
        "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/telemetry-formulas-and-status-mobile-light-parent-guard.json",
        "sha256": "d20b394d7dd8d2e595fe96d2c897799c1613b79d0d26c5e8953c712c71cb4755"
      },
      {
        "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/telemetry-formulas-and-status-mobile-light-service.log",
        "sha256": "ef8fda5f428fe19b4e2326bab6ae5bb5a0fa1a795ebd9e967a0494791a9b8d6c"
      },
      {
        "file": "/tmp/agf-a3-6d47f8db-native-code-recovery-v2/telemetry-formulas-and-status-mobile-light-cleanup.json",
        "sha256": "bf09bf970a502932f450fe5491dc89080e1b9883527a89c467b6f508acbb83ba"
      },
      {
        "file": "/home/dev/s/tmp/agf-a3-h7gr-stop-20261031T222615Z/report.json",
        "sha256": "607667fd4840b10d8c56609ae6a8b0cb4a6f60d9ba43b4d5f9083c670995e52f"
      },
      {
        "file": "/home/dev/s/tmp/agf-a3-h7gr-stop-20261031T222615Z/manifest.json",
        "sha256": "4f21beb5be6ab140ca7570243f8eb4ba3f0da6ee00fd7ea89acbcd3cf51eb6bb"
      },
      {
        "file": "/home/dev/s/tmp/agf-a3-h7gr-stop-20261031T222615Z/journal.txt",
        "sha256": "99522dbe498488968c456417f507e3e865b30144ec8a821d259a0b0dc90e63de"
      }
    ],
    "unattemptedIds": [
      "telemetry-formulas-and-status-mobile-dark",
      "telemetry-three-row-table-desktop-light",
      "telemetry-three-row-table-desktop-dark",
      "telemetry-three-row-table-mobile-light",
      "telemetry-three-row-table-mobile-dark",
      "telemetry-provenance-and-limits-desktop-light",
      "telemetry-provenance-and-limits-desktop-dark",
      "telemetry-provenance-and-limits-mobile-light",
      "telemetry-provenance-and-limits-mobile-dark",
      "telemetry-log-controls-desktop-light",
      "telemetry-log-controls-desktop-dark",
      "telemetry-log-controls-mobile-light",
      "telemetry-log-controls-mobile-dark"
    ]
  }
}
````
