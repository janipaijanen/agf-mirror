# Disclaimer

*Status: Released (v2.1 documentation).*

AGF is a governance framework for AI-assisted software development. It gives
you files, roles, review gates, coordination records, and validation tooling
that you can adapt to your own project. It does not make decisions for you.

## No professional advice

AGF is not legal, security, compliance, privacy, or operational-risk advice.
The framework can help you make decisions visible and reviewable, but you are
responsible for adapting its rules to your own organisation, contracts,
regulatory duties, and risk profile.

## Use on authorized systems

Run AI assistants, validators, CI jobs, repository-changing scripts, and
history-changing procedures only on systems, repositories, and data where you
have the necessary authorization. If a procedure could rewrite history, expose
private data, change production behavior, or affect other people's work, stop
and get the appropriate human approval before running it.

## Your responsibility

You are responsible for deciding whether AGF fits your project, reviewing AI
output before it is committed or merged, backing up before risky operations,
and complying with the obligations that apply to your environment. Treat the
templates and examples as starting points, not as proof that your project is
safe, compliant, or ready to ship.

## AI and tooling limits

AGF validators and agent roles are aids. They can catch malformed records,
missing evidence, broken links, or known policy violations, but they are not
proof of correctness, security, compliance, or absence of defects. Human
review remains responsible for accepting risk and deciding what gets merged.

The project is developed independently. Unless explicitly stated, AGF has no
endorsement, dependency, or sponsorship relationship with the tool, model, or
runtime vendors mentioned in the documentation.

## No warranty

Framework content is provided as-is under the Creative Commons Attribution
ShareAlike 4.0 license. See the
[CC BY-SA 4.0 legal code](https://creativecommons.org/licenses/by-sa/4.0/legalcode)
for the warranty exclusions and liability limitations that apply to that
licensed material. Tooling code is licensed separately under the terms stated
in the repository.
