{ name = "my-project"
, dependencies =
  [ "console"
  , "effect"
  , "halogen"
  , "prelude"
  , "psci-support"
  , "purescript-halogen-leaflet"
  -- , "purescript-leafletjs-halogen"
  ]
, packages = ./packages.dhall
, sources = [ "src/**/*.purs", "test/**/*.purs" ]
}
