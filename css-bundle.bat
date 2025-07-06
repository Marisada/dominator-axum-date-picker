@echo off

set css_path=volume\pwa\css

echo create %css_path%\picker.css
grass -s compressed picker.scss %css_path%\picker.css

echo:
reload