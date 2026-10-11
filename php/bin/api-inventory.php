<?php

declare(strict_types=1);

require_once __DIR__ . '/../vendor/autoload.php';

function defaultInventory(mixed $value): mixed
{
    if ($value instanceof UnitEnum) {
        return ['enum' => $value::class, 'case' => $value->name];
    }
    return is_array($value) ? array_map(defaultInventory(...), $value) : $value;
}

/** @return array<string, mixed> */
function parameterInventory(ReflectionParameter $parameter): array
{
    $entry = [
        'type' => (string) $parameter->getType(),
        'optional' => $parameter->isOptional(),
        'variadic' => $parameter->isVariadic(),
        'byReference' => $parameter->isPassedByReference(),
        'hasDefault' => $parameter->isDefaultValueAvailable(),
    ];
    if ($parameter->isDefaultValueAvailable()) {
        $entry['default'] = defaultInventory($parameter->getDefaultValue());
    }
    return $entry;
}

// Tests include the reflection helpers to prove that default-only drift is visible.
if (realpath($_SERVER['SCRIPT_FILENAME']) !== __FILE__) {
    return;
}

// Enumerate installed PSR-4 source files independently of the shared model/generator.
$inventory = [];
foreach (glob(__DIR__ . '/../src/*.php') as $file) {
    $name = pathinfo($file, PATHINFO_FILENAME);
    $reflection = new ReflectionClass('Slackblocks\\' . $name);
    $entry = [
        'kind' => $reflection->isEnum() ? 'enum' : ($reflection->isInterface() ? 'interface' : 'class'),
        'readonly' => $reflection->isReadOnly(),
        'interfaces' => $reflection->getInterfaceNames(),
        'properties' => [],
        'methods' => [],
        'constants' => [],
    ];
    foreach ($reflection->getProperties(ReflectionProperty::IS_PUBLIC) as $property) {
        if ($property->getDeclaringClass()->getName() !== $reflection->getName()) {
            continue;
        }
        $entry['properties'][$property->getName()] = [
            'type' => (string) $property->getType(),
            'readonly' => $property->isReadOnly(),
        ];
    }
    foreach ($reflection->getMethods(ReflectionMethod::IS_PUBLIC) as $method) {
        if ($method->getDeclaringClass()->getName() !== $reflection->getName()) {
            continue;
        }
        $signature = [
            'static' => $method->isStatic(),
            'return' => (string) $method->getReturnType() === 'self' ? $reflection->getName() : (string) $method->getReturnType(),
            'byReference' => $method->returnsReference(),
            'parameters' => [],
        ];
        foreach ($method->getParameters() as $parameter) {
            $signature['parameters'][$parameter->getName()] = parameterInventory($parameter);
        }
        $entry['methods'][$method->getName()] = $signature;
    }
    foreach ($reflection->getReflectionConstants(ReflectionClassConstant::IS_PUBLIC) as $constant) {
        if ($constant->getDeclaringClass()->getName() !== $reflection->getName()) {
            continue;
        }
        $value = $constant->getValue();
        $entry['constants'][$constant->getName()] = $value instanceof BackedEnum ? $value->value : $value;
    }
    // Interface order is an engine detail; normalize it across PHP versions.
    sort($entry['interfaces']);
    $inventory[$name] = $entry;
}
ksort($inventory);
echo json_encode($inventory, JSON_THROW_ON_ERROR | JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_PRESERVE_ZERO_FRACTION) . "\n";
