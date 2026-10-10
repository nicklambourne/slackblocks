<?php

declare(strict_types=1);

require __DIR__.'/../vendor/autoload.php';

// Enumerate installed PSR-4 source files independently of the shared model/generator.
$inventory = [];
foreach (glob(__DIR__.'/../src/*.php') as $file) {
    $name = pathinfo($file, PATHINFO_FILENAME);
    $reflection = new ReflectionClass('Slackblocks\\'.$name);
    $entry = ['kind'=>$reflection->isEnum() ? 'enum' : ($reflection->isInterface() ? 'interface' : 'class'), 'readonly'=>$reflection->isReadOnly(), 'interfaces'=>$reflection->getInterfaceNames(), 'properties'=>[], 'methods'=>[], 'constants'=>[]];
    foreach ($reflection->getProperties(ReflectionProperty::IS_PUBLIC) as $property) {
        if ($property->getDeclaringClass()->getName() !== $reflection->getName()) { continue; }
        $entry['properties'][$property->getName()] = ['type'=>(string)$property->getType(), 'readonly'=>$property->isReadOnly()];
    }
    foreach ($reflection->getMethods(ReflectionMethod::IS_PUBLIC) as $method) {
        if ($method->getDeclaringClass()->getName() !== $reflection->getName()) { continue; }
        $entry['methods'][$method->getName()] = ['static'=>$method->isStatic(), 'return'=>((string)$method->getReturnType() === 'self' ? $reflection->getName() : (string)$method->getReturnType()), 'parameters'=>[]];
        foreach ($method->getParameters() as $parameter) {
            $entry['methods'][$method->getName()]['parameters'][$parameter->getName()] = ['type'=>(string)$parameter->getType(), 'optional'=>$parameter->isOptional(), 'variadic'=>$parameter->isVariadic()];
        }
    }
    foreach ($reflection->getReflectionConstants(ReflectionClassConstant::IS_PUBLIC) as $constant) {
        if ($constant->getDeclaringClass()->getName() !== $reflection->getName()) { continue; }
        $value = $constant->getValue();
        $entry['constants'][$constant->getName()] = $value instanceof BackedEnum ? $value->value : $value;
    }
    // Interface order is an engine detail; normalize it across PHP versions.
    sort($entry['interfaces']);
    $inventory[$name] = $entry;
}
ksort($inventory);
echo json_encode($inventory, JSON_THROW_ON_ERROR | JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
