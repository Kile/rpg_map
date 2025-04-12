import os
from docutils.parsers.rst import Directive
from docutils.core import publish_doctree

class ProcessedReadme(Directive):
    required_arguments = 1
    option_spec = {'end-before': str, 'replace': lambda x: tuple(s.strip() for s in x.split(','))}

    def run(self):
        filename = self.arguments[0]
        end_before = self.options.get('end-before')
        replacements = self.options.get('replace', ())
        replacement_dict = {}
        for item in replacements:
            if '=' in item:
                key, value = item.split('=', 1)
                replacement_dict[key.strip()] = value.strip()

        on_rtd = os.environ.get('READTHEDOCS') == 'True'
        if on_rtd:
            resolved_filename = '../../' + filename
        else:
            resolved_filename = '../' + filename # Adjust local path as needed

        try:
            with open(resolved_filename, 'r') as f:
                content = f.read()
        except FileNotFoundError as e:
            return [self.reporter.error(f'File not found: {resolved_filename}')]

        if end_before:
            content = content.split(end_before)[0].strip()

        for old, new in replacement_dict.items():
            content = content.replace(old, new)

        # Parse the modified content into a document tree
        document = publish_doctree(content, settings_overrides={'output_encoding': 'unicode'})

        # Directly return the children of the parsed document
        return list(document.children)

def setup(app):
    app.add_directive('processed_readme', ProcessedReadme)