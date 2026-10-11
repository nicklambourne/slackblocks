<?php

declare(strict_types=1);

require __DIR__ . '/../../php/vendor/autoload.php';

// Reflection keeps signatures, defaults, constants and handwritten helpers tied
// to the actual public PHP API. Shared metadata supplies only page grouping.
$metadata = json_decode(file_get_contents(__DIR__ . '/../../php/generated/reference.json'), true, flags: JSON_THROW_ON_ERROR);
$domains = [];
foreach (['types', 'interfaces', 'enums'] as $group) {
    foreach ($metadata[$group] as $entry) {
        $domains[$entry['name']] = $group === 'types' ? $entry['package'] : 'core';
    }
}
foreach (['Accordion', 'AccordionSection', 'Paginator', 'Builder', 'Color'] as $name) {
    $domains[$name] = 'component';
}
foreach (['Value', 'JsonObject', 'RichTextStyle', 'Version'] as $name) {
    $domains[$name] = 'core';
}
foreach (['ValidationError', 'ErrorCategory'] as $name) {
    $domains[$name] = 'error';
}
function prose(string $text): string
{
    return str_replace(['{', '}', '|'], ['&#123;', '&#125;', '\\|'], htmlspecialchars($text, ENT_NOQUOTES));
}
function documentation(string|false $comment): array
{
    $body = preg_replace('/\A\s*\/\*\*|\*\/\s*\z/', '', $comment ?: '') ?? '';
    $lines = preg_split('/\R/', $body) ?: [];
    $summary = [];
    $params = [];
    $see = [];
    $returns = '';
    $variable = '';
    $throws = [];
    foreach ($lines as $line) {
        $line = trim(preg_replace('/^\s*\* ?/', '', $line) ?? '');
        if (preg_match('/^@param\s+(.+?)\s+\$(\w+)\s*(.*)$/', $line, $match)) {
            $params[$match[2]] = prose($match[1] . ' — ' . $match[3]);
        } elseif (preg_match('/^@return\s+(.*)$/', $line, $match)) {
            $returns = prose($match[1]);
        } elseif (preg_match('/^@var\s+(\S+)/', $line, $match)) {
            $variable = $match[1];
        } elseif (preg_match('/^@throws\s+(\S+)\s+(.+)$/', $line, $match)) {
            $throws[] = ['type' => shortType($match[1]), 'doc' => prose($match[2])];
        } elseif (preg_match('/^@see\s+(https:\/\/\S+)/', $line, $match)) {
            $see[] = ['label' => 'Slack reference', 'url' => $match[1]];
        } elseif ($line !== '' && !str_starts_with($line, '@')) {
            $summary[] = $line;
        }
    }
    return [prose(implode(' ', $summary)), $params, $returns, $see, $variable, $throws];
}
function shortType(string $type): string
{
    return str_replace('Slackblocks\\', '', $type);
}
function defaultValue(mixed $value): string
{
    if ($value instanceof BackedEnum) {
        return shortType($value::class) . '::' . $value->name;
    }
    if ($value === []) {
        return '[]';
    }
    return var_export($value, true);
}
$types = [];
foreach (glob(__DIR__ . '/../../php/src/*.php') as $file) {
    $name = pathinfo($file, PATHINFO_FILENAME);
    if (!isset($domains[$name])) {
        throw new LogicException('Missing PHP reference domain: ' . $name);
    }
    $class = new ReflectionClass('Slackblocks\\' . $name);
    [$doc, , , $see] = documentation($class->getDocComment());
    if ($class->isSubclassOf(Slackblocks\Value::class)) {
        $doc .= ' Readonly properties contain normalized values. See [`Value`](ref:Value) for inherited `with()`, `fromArray()`, `fromJson()`, `toArray()`, `toJson()` and `jsonSerialize()`.';
    }
    $members = [];
    $constants = [];
    foreach ($class->getProperties(ReflectionProperty::IS_PUBLIC) as $property) {
        if ($property->getDeclaringClass()->getName() !== $class->getName()) {
            continue;
        }
        [$description, , , , $variable] = documentation($property->getDocComment());
        if ($class->isEnum()) {
            $description = $property->getName() === 'name' ? 'PHP case name, as declared in the enum.' : 'Slack wire string represented by this case.';
        }
        $collection = str_contains($variable, '<') ? '/** @var ' . $variable . ' */' . "\n" : '';
        $members[] = ['name' => '$' . $property->getName(), 'signature' => $collection . 'public ' . ($property->isReadOnly() ? 'readonly ' : '') . shortType((string) $property->getType()) . ' $' . $property->getName() . ';', 'doc' => $description, 'params' => [], 'returns' => '', 'throws' => []];
    }
    foreach ($class->getMethods(ReflectionMethod::IS_PUBLIC) as $method) {
        if ($method->getDeclaringClass()->getName() !== $class->getName()) {
            continue;
        }
        [$description, $parameters, $returns, , , $throws] = documentation($method->getDocComment());
        if ($class->isEnum() && $method->isInternal()) {
            $description = match ($method->getName()) {
                'cases' => 'Returns every enum case in declaration order.',
                'from' => 'Resolves a Slack wire string to its enum case, throwing if it is unknown.',
                'tryFrom' => 'Resolves a Slack wire string to its enum case, returning null if it is unknown.',
            };
            $parameters = ['value' => 'Slack wire string to resolve.'];
            $returns = $method->getName() === 'cases' ? 'list&lt;' . $name . '&gt; Declared cases.' : $name . ($method->getName() === 'tryFrom' ? '|null' : '') . ' Matching case.';
            $throws = $method->getName() === 'from' ? [['type' => 'ValueError', 'doc' => 'The wire string has no matching case.']] : [];
        }
        $arguments = [];
        $params = [];
        foreach ($method->getParameters() as $parameter) {
            $argument = shortType((string) $parameter->getType()) . ' ' . ($parameter->isPassedByReference() ? '&' : '') . ($parameter->isVariadic() ? '...' : '') . '$' . $parameter->getName();
            if ($parameter->isDefaultValueAvailable()) {
                $argument .= ' = ' . defaultValue($parameter->getDefaultValue());
            }
            $arguments[] = $argument;
            $params[] = ['name' => $parameter->getName(), 'doc' => $parameters[$parameter->getName()] ?? ($parameter->isOptional() ? 'Optional.' : 'Required.')];
        }
        $signature = 'public ' . ($method->isStatic() ? 'static ' : '') . 'function ' . $method->getName() . '(' . implode(', ', $arguments) . ')' . ($method->hasReturnType() ? ': ' . shortType((string) $method->getReturnType()) : '');
        if ($method->isConstructor() && $class->isSubclassOf(Slackblocks\Value::class)) {
            $description = 'Construct a validated immutable value. ' . $description;
            $throws = [['type' => 'ValidationError', 'doc' => 'A content or contextual rule fails. PHP rejects incompatible call signatures with native errors.']];
        }
        $members[] = ['name' => $method->getName(), 'signature' => $signature, 'doc' => $description, 'params' => $params, 'returns' => $returns, 'throws' => $throws];
    }
    foreach ($class->getReflectionConstants(ReflectionClassConstant::IS_PUBLIC) as $constant) {
        if ($constant->getDeclaringClass()->getName() !== $class->getName()) {
            continue;
        }
        [$description] = documentation($constant->getDocComment());
        $value = $constant->getValue();
        $constants[] = ['name' => $constant->getName(), 'wire' => defaultValue($value instanceof BackedEnum ? $value->value : $value), 'doc' => $description];
    }
    $types[] = ['name' => $name, 'package' => $domains[$name], 'doc' => $doc, 'see' => $see, 'members' => $members, 'constants' => $constants];
}
echo json_encode(['types' => $types], JSON_THROW_ON_ERROR | JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES);
