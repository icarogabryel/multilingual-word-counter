# Multilingual word Counter

A simple Python module created in Rust that counts the number of words in a given text.

## Building the Module

To build the module, use the following command with virtual environment activated:

```bash
maturin develop
```

## Using the Module

You can use the module in your Python code as follows:

```python
from word_counter import count_words


text = "Hello, world! This is a test."
word_count = count_words(text)
print(f"Number of words: {word_count}")
```
