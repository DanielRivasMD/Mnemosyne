
################################################################################

_default:
  @just --list

################################################################################

# build release
build:
  cargo build --release

################################################################################

# run cargo test & write to out/testout
test:
  script -q /dev/null cargo test | tee outTest/testout

################################################################################

# run cargo debug
debug:
  cargo build
  ./target/debug/mnemosyne

################################################################################

# run cargo clippy
clippy:
  cargo clippy

################################################################################

# link Mnemosyne to PATH
@ link:
  ln -svf $HOME/Factorem/Mnemosyne/target/debug/mnemosyne $HOME/bin/toolLinks/

################################################################################

