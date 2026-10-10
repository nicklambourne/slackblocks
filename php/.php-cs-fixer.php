<?php

$finder = PhpCsFixer\Finder::create()->in([__DIR__.'/src', __DIR__.'/tests', __DIR__.'/examples', __DIR__.'/typecheck'])->notName('Schema.php');
return (new PhpCsFixer\Config())->setRules(['@PER-CS2.0' => true])->setFinder($finder);
