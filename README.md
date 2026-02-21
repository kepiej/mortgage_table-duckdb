# DuckDB Rust extension template
This is an **experimental** template for Rust based extensions based on the C Extension API of DuckDB. The goal is to
turn this eventually into a stable basis for pure-Rust DuckDB extensions that can be submitted to the Community extensions
repository

Features:
- No DuckDB build required
- No C++ or C code required
- CI/CD chain preconfigured
- (Coming soon) Works with community extensions

## Cloning

Clone the repo with submodules

```shell
git clone --recurse-submodules <repo>
```

## Dependencies
In principle, these extensions can be compiled with the Rust toolchain alone. However, this template relies on some additional
tooling to make life a little easier and to be able to share CI/CD infrastructure with extension templates for other languages:

- Python3
- Python3-venv
- [Make](https://www.gnu.org/software/make)
- Git

Installing these dependencies will vary per platform:
- For Linux, these come generally pre-installed or are available through the distro-specific package manager.
- For MacOS, [homebrew](https://formulae.brew.sh/).
- For Windows, [chocolatey](https://community.chocolatey.org/).

## Building
After installing the dependencies, building is a two-step process. Firstly run:
```shell
make configure
```
This will ensure a Python venv is set up with DuckDB and DuckDB's test runner installed. Additionally, depending on configuration,
DuckDB will be used to determine the correct platform for which you are compiling.

Then, to build the extension run:
```shell
make debug
```
This delegates the build process to cargo, which will produce a shared library in `target/debug/<shared_lib_name>`. After this step,
a script is run to transform the shared library into a loadable extension by appending a binary footer. The resulting extension is written
to the `build/debug` directory.

To create optimized release binaries, simply run `make release` instead.

### Running the extension
To run the extension code, start `duckdb` with `-unsigned` flag. This will allow you to load the local extension file.

```sh
duckdb -unsigned
```

After loading the extension by the file path, you can use the functions provided by the extension (in this case, `rusty_quack()`).

```sql
LOAD './build/debug/extension/rusty_quack/rusty_quack.duckdb_extension';
SELECT * FROM mortgage_table(100000.0, 240, 1.8);
```

```
┌───────┬───────────────────┬───────────────────┬────────────────────┐
│ month │     payments      │      capital      │      interest      │
│ int64 │      double       │      double       │       double       │
├───────┼───────────────────┼───────────────────┼────────────────────┤
│     1 │ 998.4914635253558 │ 833.3333333333334 │ 165.15813019202244 │
│     2 │ 997.1151457737556 │ 833.3333333333334 │  163.7818124404222 │
│     3 │ 995.7388280221554 │ 833.3333333333334 │ 162.40549468882205 │
│     4 │ 994.3625102705553 │ 833.3333333333334 │  161.0291769372219 │
│     5 │  992.986192518955 │ 833.3333333333334 │ 159.65285918562165 │
│     6 │ 991.6098747673549 │ 833.3333333333334 │  158.2765414340215 │
│     7 │ 990.2335570157547 │ 833.3333333333334 │ 156.90022368242137 │
│     8 │ 988.8572392641545 │ 833.3333333333334 │ 155.52390593082112 │
│     9 │ 987.4809215125543 │ 833.3333333333334 │ 154.14758817922097 │
│    10 │ 986.1046037609542 │ 833.3333333333334 │ 152.77127042762083 │
│    11 │  984.728286009354 │ 833.3333333333334 │ 151.39495267602058 │
│    12 │ 983.3519682577538 │ 833.3333333333334 │ 150.01863492442044 │
│    13 │ 981.9756505061537 │ 833.3333333333334 │  148.6423171728203 │
│    14 │ 980.5993327545534 │ 833.3333333333334 │ 147.26599942122004 │
│    15 │ 979.2230150029533 │ 833.3333333333334 │  145.8896816696199 │
│    16 │ 977.8466972513531 │ 833.3333333333334 │ 144.51336391801976 │
│    17 │ 976.4703794997529 │ 833.3333333333334 │  143.1370461664195 │
│    18 │ 975.0940617481527 │ 833.3333333333334 │ 141.76072841481937 │
│    19 │ 973.7177439965526 │ 833.3333333333334 │ 140.38441066321923 │
│    20 │ 972.3414262449523 │ 833.3333333333334 │ 139.00809291161897 │
│     · │         ·         │         ·         │          ·         │
│     · │         ·         │         ·         │          ·         │
│     · │         ·         │         ·         │          ·         │
│   101 │ 860.8596883653373 │ 833.3333333333334 │  27.52635503200395 │
│   102 │ 859.4833706137372 │ 833.3333333333334 │ 26.150037280403808 │
│   103 │ 858.1070528621369 │ 833.3333333333334 │ 24.773719528803554 │
│   104 │ 856.7307351105368 │ 833.3333333333334 │ 23.397401777203413 │
│   105 │ 855.3544173589365 │ 833.3333333333334 │  22.02108402560316 │
│   106 │ 853.9780996073364 │ 833.3333333333334 │  20.64476627400302 │
│   107 │ 852.6017818557361 │ 833.3333333333334 │ 19.268448522402764 │
│   108 │  851.225464104136 │ 833.3333333333334 │ 17.892130770802623 │
│   109 │ 849.8491463525359 │ 833.3333333333334 │ 16.515813019202483 │
│   110 │ 848.4728286009356 │ 833.3333333333334 │ 15.139495267602229 │
│   111 │ 847.0965108493355 │ 833.3333333333334 │ 13.763177516002088 │
│   112 │ 845.7201930977352 │ 833.3333333333334 │ 12.386859764401834 │
│   113 │ 844.3438753461351 │ 833.3333333333334 │ 11.010542012801693 │
│   114 │ 842.9675575945349 │ 833.3333333333334 │  9.634224261201553 │
│   115 │ 841.5912398429347 │ 833.3333333333334 │  8.257906509601298 │
│   116 │ 840.2149220913345 │ 833.3333333333334 │  6.881588758001158 │
│   117 │ 838.8386043397343 │ 833.3333333333334 │  5.505271006400903 │
│   118 │ 837.4622865881341 │ 833.3333333333334 │  4.128953254800763 │
│   119 │  836.085968836534 │ 833.3333333333334 │ 2.7526355032006222 │
│   120 │ 834.7096510849337 │ 833.3333333333334 │  1.376317751600368 │
├───────┴───────────────────┴───────────────────┴────────────────────┤
│ 120 rows (40 shown)                                      4 columns │
└────────────────────────────────────────────────────────────────────┘
```

## Testing
This extension uses the DuckDB Python client for testing. This should be automatically installed in the `make configure` step.
The tests themselves are written in the SQLLogicTest format, just like most of DuckDB's tests. A sample test can be found in
`test/sql/<extension_name>.test`. To run the tests using the *debug* build:

```shell
make test_debug
```

or for the *release* build:
```shell
make test_release
```

### Version switching
Testing with different DuckDB versions is really simple:

First, run
```
make clean_all
```
to ensure the previous `make configure` step is deleted.

Then, run
```
DUCKDB_TEST_VERSION=v1.3.2 make configure
```
to select a different duckdb version to test with

Finally, build and test with
```
make debug
make test_debug
```

### Known issues
This is a bit of a footgun, but the extensions produced by this template may (or may not) be broken on windows on python3.11
with the following error on extension load:
```shell
IO Error: Extension '<name>.duckdb_extension' could not be loaded: The specified module could not be found
```
This was resolved by using python 3.12
