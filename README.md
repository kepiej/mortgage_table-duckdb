# Mortgage table extension
This is an extension for duckdb to calculate monthly payments for mortgages.

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

After loading the extension by the file path, you can use the functions provided by the extension (in this case, `mortgage_table()` with arguments principal, number of months, yearly interest rate (in %) and mortgage type (one of "FixedMensualities", "FixedCapital", "VariableLinearCapital init_pay" where init_pay is the desired initial payment)).

```sql
LOAD './build/debug/extension/mortgage_table/mortgage_table.duckdb_extension';
SELECT * FROM mortgage_table(100000.0, 240, 1.8, "FixedMensualities");
```

```
┌───────┬────────────────────┬────────────────────┬────────────────────┐
│ month │      payments      │      capital       │      interest      │
│ int64 │       double       │       double       │       double       │
├───────┼────────────────────┼────────────────────┼────────────────────┤
│     1 │ 505.02646071266395 │  339.8683305206415 │ 165.15813019202244 │
│     2 │ 505.02646071266395 │ 340.42965070044426 │  164.5968100122197 │
│     3 │ 505.02646071266395 │ 340.99189794616035 │  164.0345627665036 │
│     4 │ 505.02646071266395 │ 341.55507378891457 │ 163.47138692374938 │
│     5 │ 505.02646071266395 │  342.1191797623603 │ 162.90728095030363 │
│     6 │ 505.02646071266395 │  342.6842174026841 │ 162.34224330997984 │
│     7 │ 505.02646071266395 │  343.2501882486095 │ 161.77627246405444 │
│     8 │ 505.02646071266395 │  343.8170938414015 │ 161.20936687126243 │
│     9 │ 505.02646071266395 │  344.3849357248705 │ 160.64152498779345 │
│    10 │ 505.02646071266395 │  344.9537154453767 │ 160.07274526728725 │
│    11 │ 505.02646071266395 │ 345.52343455183416 │  159.5030261608298 │
│    12 │ 505.02646071266395 │ 346.09409459571526 │ 158.93236611694869 │
│    13 │ 505.02646071266395 │ 346.66569713105457 │ 158.36076358160938 │
│    14 │ 505.02646071266395 │ 347.23824371445335 │  157.7882169982106 │
│    15 │ 505.02646071266395 │ 347.81173590508376 │ 157.21472480758018 │
│    16 │ 505.02646071266395 │   348.386175264693 │ 156.64028544797094 │
│    17 │ 505.02646071266395 │  348.9615633576077 │ 156.06489735505625 │
│    18 │ 505.02646071266395 │ 349.53790175073794 │   155.488558961926 │
│    19 │ 505.02646071266395 │  350.1151920135819 │ 154.91126869908203 │
│    20 │ 505.02646071266395 │ 350.69343571822975 │  154.3330249944342 │
│     · │          ·         │          ·         │          ·         │
│     · │          ·         │          ·         │          ·         │
│     · │          ·         │          ·         │          ·         │
│   221 │ 505.02646071266395 │  488.6304347194686 │ 16.396025993195337 │
│   222 │ 505.02646071266395 │ 489.43744760900046 │  15.58901310366349 │
│   223 │ 505.02646071266395 │   490.245793345931 │ 14.780667366732928 │
│   224 │ 505.02646071266395 │ 491.05547413156626 │  13.97098658109769 │
│   225 │ 505.02646071266395 │  491.8664921708475 │ 13.159968541816454 │
│   226 │ 505.02646071266395 │ 492.67884967235796 │ 12.347611040305992 │
│   227 │ 505.02646071266395 │  493.4925488483284 │  11.53391186433555 │
│   228 │ 505.02646071266395 │ 494.30759191464324 │ 10.718868798020708 │
│   229 │ 505.02646071266395 │ 495.12398109084666 │  9.902479621817292 │
│   230 │ 505.02646071266395 │ 495.94171860014865 │    9.0847421125153 │
│   231 │ 505.02646071266395 │  496.7608066694308 │  8.265654043233155 │
│   232 │ 505.02646071266395 │ 497.58124752925283 │  7.445213183411113 │
│   233 │ 505.02646071266395 │  498.4030434138583 │  6.623417298805634 │
│   234 │ 505.02646071266395 │ 499.22619656118076 │  5.800264151483191 │
│   235 │ 505.02646071266395 │ 500.05070921284994 │ 4.9757514998140095 │
│   236 │ 505.02646071266395 │ 500.87658361419784 │  4.149877098466106 │
│   237 │ 505.02646071266395 │ 501.70382201426474 │  3.322638698399203 │
│   238 │ 505.02646071266395 │  502.5324266658054 │ 2.4940340468585305 │
│   239 │ 505.02646071266395 │ 503.36239982529526 │ 1.6640608873686915 │
│   240 │ 505.02646071266395 │  504.1937437529364 │ 0.8327169597275201 │
├───────┴────────────────────┴────────────────────┴────────────────────┤
│ 240 rows (40 shown)                                        4 columns │
└──────────────────────────────────────────────────────────────────────┘
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
