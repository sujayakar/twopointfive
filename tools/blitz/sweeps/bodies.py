"""The files a sweep reads: every body file of the rules, whatever it is called, with the module it is of."""
import glob, os

def bodies(root, module=None):
    """(path, module, path under the rules) of each .rs file in a module's folder, its types apart; of one module where it is named"""
    for path in sorted(glob.glob(os.path.join(root, module or '*', '**', '*.rs'), recursive=True)):
        rel = os.path.relpath(path, root).replace(os.sep, '/')
        if os.path.basename(path) == 'types.rs': continue
        yield path, rel.split('/')[0], rel
