from distutils.core import setup
setup(
    name = 'bm25fast',         # How you named your package folder (MyLib)
    packages = ['bm25fast'],   # Chose the same as "name"
    version = '0.1.1',      # Start with a small number and increase it with every change you make
    license='MIT',        # Chose a license from here: https://help.github.com/articles/licensing-a-repository
    description = 'Implements the BM25 document ranking system in Rust. This is not a particularly slow algorithm, even in native Python, but this implementation increases the speed of both loading the initial documents and of recall greatly compared to similar libraries. It also offers runtime weight configurability that could linked to an optimisation procedure.',   # Give a short description about your library
    author = 'Tim Attwell',                   # Type in your name
    author_email = 'tim.attwell@hotmail.co.uk',      # Type in your E-Mail
    url = 'https://github.com/timattwell',   # Provide either the link to your github or to your website
    download_url = 'https://github.com/timattwell/bm25-rust',    # I explain this later on
    keywords = [
        'b25fast',
        'BM25', 
        'BM25Plus', 
        'Document', 
        'Ranking', 
        'NLP', 
        'Relevance', 
        'Rust',
        'language',
    ],   # Keywords that define your package best
    install_requires=[            # I get to this in a second
        'validators',
        'beautifulsoup4',
    ],
    classifiers=[
        'Development Status :: 3 - Alpha',      # Chose either "3 - Alpha", "4 - Beta" or "5 - Production/Stable" as the current state of your package
        'Intended Audience :: Developers',      # Define that your audience are developers
        'Topic :: Software Development :: Build Tools',
        'License :: OSI Approved :: MIT License',   # Again, pick a license
        'Programming Language :: Python :: 3',      #Specify which pyhton versions that you want to support
        'Programming Language :: Python :: 3.4',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
    ],
)