import rpg_map
# -- Path setup --------------------------------------------------------------

# sys.path.insert(0, os.path.abspath('..'))

# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'rpg_map'
copyright = '2025, Kile'
author = 'Kile'
release = '0.0.2'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.autodoc',  # To automatically generate docs from docstrings
    'sphinx.ext.napoleon', # To support NumPy and Google style docstrings
    'sphinx.ext.intersphinx', # To link to other projects' documentation
    'sphinx.ext.viewcode',  # To include links to the source code
    'sphinx_rtd_theme',     # To use the Read the Docs theme
]

templates_path = ['_templates']
exclude_patterns = []



# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']
