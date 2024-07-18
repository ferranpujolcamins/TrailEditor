{- 
 - Copyright (C) 2021 Ferran Pujol Camins
 - 
 - This program is free software: you can redistribute it and/or modify
 - it under the terms of the GNU Affero General Public License as
 - published by the Free Software Foundation, either version 3 of the
 - License, or (at your option) any later version.
 - 
 - This program is distributed in the hope that it will be useful,
 - but WITHOUT ANY WARRANTY; without even the implied warranty of
 - MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 - GNU Affero General Public License for more details.
 - 
 - You should have received a copy of the GNU Affero General Public License
 - along with this program.  If not, see <http://www.gnu.org/licenses/>.
 -}

module MainComponent
where

import Prelude
import Data.Maybe (Maybe(..), maybe)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import LeafletComponent as LC
import Control.Monad.Aff
import Control.Monad.Aff.AVar (AVAR)
import Control.Monad.Eff.Exception (EXCEPTION)
import Leaflet (LEAFLET, LatLng, lat, lng, Zoom, MouseEvent (..))
import Leaflet.TileLayer as TileLayer
import Data.Tuple (Tuple (..), fst)
import Util (formatGeo)
import Data.Array (elem, catMaybes)

data Query a
  = HandleLeaflet LC.Message a
  | Initialize a

type State =
  { view :: Maybe (Tuple LatLng Zoom)
  , mousePos :: Maybe LatLng
  }

data Slot = LeafletSlot
derive instance eqLeafletSlot :: Eq Slot
derive instance ordLeafletSlot :: Ord Slot

ui :: forall eff
    . H.Component
       HH.HTML
       Query
       Unit
       Void
       (Aff (avar :: AVAR, leaflet :: LEAFLET, err :: EXCEPTION | eff))
ui =
  H.lifecycleParentComponent
    { initialState: const initialState
    , render
    , eval
    , receiver: const Nothing
    , initializer: Just (H.action Initialize)
    , finalizer: Nothing
    }
  where

  initialState :: State
  initialState =
    { view: Nothing
    , mousePos: Nothing
    }

  render :: State
         -> H.ParentHTML
              Query
              LC.Query
              Slot
              (Aff (avar :: AVAR, leaflet :: LEAFLET, err :: EXCEPTION | eff))

  render state =
    HH.div_
      [ HH.slot LeafletSlot (LC.ui "slippy_map") unit (HE.input HandleLeaflet)
      , HH.div
        [ HP.class_ $ H.ClassName "overlay"
        ]
        $ catMaybes
        [ case state.view of
            Nothing ->
              Nothing
            Just (Tuple ll zoom) ->
              Just $ HH.div
                [ HP.class_ $ H.ClassName "panel"
                ]
                [ HH.div [ HP.class_ $ H.ClassName "geo" ] [ HH.text $ formatGeo "N" "S" (lat ll) ]
                , HH.div [ HP.class_ $ H.ClassName "geo" ] [ HH.text $ formatGeo "E" "W" (lng ll) ]
                , HH.div [ HP.class_ $ H.ClassName "zoom" ] [ HH.text $ "zoom: " <> show zoom ]
                ]
        , case state.mousePos of
            Nothing -> Nothing
            Just ll ->
              Just $ HH.div
                [ HP.class_ $ H.ClassName "panel"
                ]
                [ HH.div [ HP.class_ $ H.ClassName "geo" ] [ HH.text $ formatGeo "N" "S" (lat ll) ]
                , HH.div [ HP.class_ $ H.ClassName "geo" ] [ HH.text $ formatGeo "E" "W" (lng ll) ]
                ]
        ]
      ]

  eval :: Query
       ~> H.ParentDSL
            State
            Query
            LC.Query
            Slot
            Void
            (Aff (avar :: AVAR, leaflet :: LEAFLET, err :: EXCEPTION | eff))
  eval = case _ of
    Initialize next -> do
      _ <- H.query LeafletSlot $ H.request
            (LC.AddTileLayer
              "//{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
              [ TileLayer.attribution "© OpenStreetMap"
              ]
            )
      pure next
    HandleLeaflet msg next -> do
      case msg of
        LC.Moved -> updateView
        LC.Zoomed -> updateView
        LC.Initialized -> updateView
        LC.MouseMoved (MouseEvent e) -> do
          H.modify _ { mousePos = Just e.latlng }
        _ -> pure unit
      pure next
  
  updateView = do
    v <- H.query LeafletSlot $ H.request LC.GetView
    H.modify _ { view = join v }
