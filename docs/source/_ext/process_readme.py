from docutils.parsers.rst import Directive
from docutils.statemachine import StringList
from docutils import nodes

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

        try:
            with open(filename, 'r') as f:
                content = f.read()
        except FileNotFoundError:
            return [self.reporter.error(f'File not found: {filename}')]

        if end_before:
            content = content.split(end_before)[0].strip()

        for old, new in replacement_dict.items():
            content = content.replace(old, new)

        lines = StringList(content.splitlines())
        node = nodes.section()  # Or a more appropriate container node
        self.state.nested_parse(lines, 0, node)
        return node.children

def setup(app):
    app.add_directive('processed_readme', ProcessedReadme)