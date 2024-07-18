let upstream =
      https://github.com/purescript/package-sets/releases/download/psc-0.14.5-20220103/packages.dhall sha256:6d8302fb12249524ab2f91282935c1750789a1f3d68dc0bcb7ee46441f91f244

in  upstream
  with purescript-halogen-leaflet =
    { dependencies =
      [ "console"
      , "aff"
      , "prelude"
      , "foreign"
      , "foreign-generic"
      , "errors"
      , "effect"
      ]
    , repo = "https://github.com/tdammers/purescript-halogen-leaflet"
    , version = "v0.1.0"
    }
  with purescript-leafletjs-halogen =
    { dependencies = [ "effect" ]
    , repo = "https://github.com/slamdata/purescript-leafletjs-halogen"
    , version = "v3.0.0"
    }
