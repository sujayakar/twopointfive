"""The files a sweep reads: every body file of the rules, whatever it is called, with the module it is of."""
import glob, os

def bodies(root, module=None):
    """(path, module, path under the rules) of each .rs file of a module, in its system's folder; of one module where it is named"""
    for path in sorted(glob.glob(os.path.join(root, '*', '**', '*.rs'), recursive=True)):
        rel = os.path.relpath(path, root).replace(os.sep, '/')
        of = rel.split('/')[1].removesuffix('.rs')
        if of not in ('mod', 'mind', 'dice', 'state_words') and module in (None, of): yield path, of, rel
