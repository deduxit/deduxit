# The workflow script of the audit's stages 3 to 5 (step 86)

One script runs a stage: a finder per unit, then a verifier on another
model, each returning its result through a schema. Its `args` name the
units (`stage5-args.json` beside this file holds stage 5's). To run or
resume a stage, write the script below to a file and call the Workflow
tool with `scriptPath`, the same `args`, and for a resume
`resumeFromRunId`: an agent that finished returns from the run's cache
as long as the script and the args are unchanged.

```js
export const meta = {
  name: 'audit-stage-full',
  description: 'Step 86 audit, one stage: a finder per unit returns its findings in full, then a verifier on another model tries to refute each one',
  phases: [
    { title: 'Find', detail: 'each finder reads its unit and returns its findings' },
    { title: 'Verify', detail: 'a verifier on another model checks every finding' },
  ],
}

const A = '/home/tux/Projects/own/deduxit-wt-audit'
const BRIEF = A + '/plan/notes/audit/brief.md'
const MODELS = { opus: 'Opus 5.5', fable: 'Fable 5.2', sonnet: 'Sonnet 5.5' }

const FINDINGS = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          dimension: { type: 'string', enum: ['idiom', 'architecture', 'performance', 'api', 'text'] },
          criterion: { type: 'string' },
          place: { type: 'string' },
          claim: { type: 'string' },
          witness: { type: 'string' },
          change: { type: 'string' },
          kind: { type: 'string', enum: ['refactor', 'API break', 'search change', 'text', 'build'] },
          value: { type: 'string' },
          cost: { type: 'string', enum: ['small', 'medium', 'large'] },
          net: { type: 'string' },
          candidate: { type: 'boolean' },
        },
        required: ['id', 'dimension', 'criterion', 'place', 'claim', 'witness', 'change', 'kind', 'value', 'cost', 'net', 'candidate'],
      },
    },
    filesReadInFull: { type: 'integer' },
    filesNotReadInFull: { type: 'array', items: { type: 'string' } },
    remarks: { type: 'string' },
  },
  required: ['unit', 'findings', 'filesReadInFull', 'filesNotReadInFull', 'remarks'],
}

const VERDICTS = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    verdicts: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          verdict: { type: 'string', enum: ['confirmed', 'candidate', 'rejected'] },
          reason: { type: 'string' },
          correction: { type: 'string' },
        },
        required: ['id', 'verdict', 'reason', 'correction'],
      },
    },
    beside: { type: 'array', items: { type: 'string' } },
  },
  required: ['unit', 'verdicts', 'beside'],
}

const dirOf = u => A + '/target/audit/units/' + u.id
const rulesOf = u => u.rules.map(r => '- ' + r).join('\n')

function finderPrompt(u) {
  const dir = dirOf(u)
  const who = MODELS[u.finder.model] + ' at ' + u.finder.effort + ' effort'
  return [
    'You are a finder of the audit of deduxit (step 86, phase B). Your unit is ' + u.id + ': ' + u.title + '. You run on ' + who + '.',
    'First read ' + BRIEF + ' in full. It says where everything is, the rules you work under, what a finding is and its form, and, under "The finder", your procedure. Follow it exactly.',
    'Your unit\'s files, each to be read in full: ' + dir + '/files.txt. Read several of them per message: every request reads your whole context again, so few requests with many reads each cost far less than many with one.',
    'Your unit\'s rules files:\n' + rulesOf(u),
    u.extra,
    'Keep your notes in ' + dir + '/notes.md as you read (append with a shell here-document every few files, not after each).',
    'Return your findings through the schema, at most 25, the most valuable first, ids F-' + u.id + '-01 onwards: for each the id, dimension, criterion, place, claim (one sentence), witness (Markdown: the lines quoted exactly in a fenced block, or the measurement with its command, or the use that fails, or the duplicate\'s two places), change, kind, value, cost, net, and whether it is a candidate (a performance claim not measured). Write no findings.md yourself. Also return the number of files you read in full, the files of your unit you did not read in full, and in remarks, in at most six sentences, what the leader should know beside the findings: what you could not judge, and what another unit should look at (name the unit).',
  ].filter(Boolean).join('\n\n')
}

function render(f) {
  return [
    '## ' + f.id,
    '- dimension: ' + f.dimension + '\n- criterion: ' + f.criterion + '\n- place: ' + f.place + '\n- kind: ' + f.kind + '\n- cost: ' + f.cost + '\n- candidate: ' + (f.candidate ? 'yes' : 'no'),
    '**Claim.** ' + f.claim,
    '**Witness.** ' + f.witness,
    '**Change.** ' + f.change,
    '**Value.** ' + f.value,
    '**Net.** ' + f.net,
  ].join('\n\n')
}

function verifierPrompt(u, found) {
  const who = MODELS[u.verifier.model] + ' at ' + u.verifier.effort + ' effort'
  return [
    'You are a verifier of the audit of deduxit (step 86, phase B). Your unit is ' + u.id + ': ' + u.title + '. You run on ' + who + '. An agent on another model read the unit and wrote the ' + found.findings.length + ' findings below; your task is to refute each of them.',
    'First read ' + BRIEF + ' in full. It says where everything is, the rules you work under, what a finding is, and, under "The verifier", your procedure and your three verdicts. Follow it exactly. Every request reads your whole context again, so check several places per message.',
    'The unit\'s rules files, where recorded choices are:\n' + rulesOf(u),
    u.verifierExtra,
    'Return your verdicts through the schema: for every finding, in order, its id, your verdict (confirmed, candidate or rejected), the reason in a short paragraph that names the places you opened, and a correction of the place, the cost or the kind (an empty string if none); and in beside, any defect of correctness you came across (an empty list if none). Write no verdicts.md yourself.',
    '# The findings of unit ' + u.id + '\n\n' + found.findings.map(render).join('\n\n'),
  ].filter(Boolean).join('\n\n')
}

const results = await pipeline(
  args.units,
  u => agent(finderPrompt(u), { label: 'find:' + u.id, phase: 'Find', schema: FINDINGS, model: u.finder.model, effort: u.finder.effort }),
  (found, u) => {
    if (!found) return { unit: u.id, failed: 'the finder returned nothing' }
    if (!found.findings.length) return { unit: u.id, found, verdicts: null }
    return agent(verifierPrompt(u, found), { label: 'verify:' + u.id, phase: 'Verify', schema: VERDICTS, model: u.verifier.model, effort: u.verifier.effort })
      .then(verdicts => ({ unit: u.id, found, verdicts }))
  },
)

return results.map((r, i) => {
  if (!r) return { unit: args.units[i].id, failed: 'a stage threw' }
  if (r.failed) return r
  const verdictOf = {}
  if (r.verdicts) r.verdicts.verdicts.forEach(v => { verdictOf[v.id] = v })
  const count = name => r.found.findings.filter(f => (verdictOf[f.id] || {}).verdict === name).length
  return {
    unit: r.unit,
    findings: r.found.findings.length,
    confirmed: count('confirmed'),
    candidate: count('candidate'),
    rejected: count('rejected'),
    unverified: r.found.findings.filter(f => !verdictOf[f.id]).length,
    filesReadInFull: r.found.filesReadInFull,
    filesNotReadInFull: r.found.filesNotReadInFull,
    remarks: r.found.remarks,
    beside: r.verdicts ? r.verdicts.beside : [],
  }
})
```
