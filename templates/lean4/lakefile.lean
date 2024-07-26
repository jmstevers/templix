import Lake
open Lake DSL

package «templix{name}» where

@[default_target]
lean_exe «templix{name}» where
    root := `Main
