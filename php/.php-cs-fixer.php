<?php

$finder = PhpCsFixer\Finder::create()->in(array_filter([__DIR__.'/src', __DIR__.'/tests', __DIR__.'/examples', __DIR__.'/typecheck'], 'is_dir'))->notName('Schema.php');
return (new PhpCsFixer\Config())->setCacheFile(__DIR__.'/.php-cs-fixer.cache')->setRules(['@PER-CS2.0' => true])->setFinder($finder);
