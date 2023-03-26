(defproject mirror_parser "TEST_ONLY"
  :description "Test parser to test and compare with main parser"
  :url "http://example.com/FIXME"
  :license {:name "MIT License"
            :url "https://github.com/burbokop/parser300b/blob/master/LICENSE"}
  :dependencies [[org.clojure/clojure "1.10.0"]
                 [org.clojure/clojurescript "1.9.495"]
                 [com.lucasbradstreet/instaparse-cljs "1.4.1.2"]
                 [org.clojure/tools.cli "1.0.214"]
                 [clj-commons/clj-yaml "1.0.26"]]
  :plugins [[lein-npm "0.6.1"]
            ;[lein-nodecljs "0.11.1"]
            ]
  :clean-targets ["out" "release", "target"]
  :npm {:dependencies [[source-map-support "0.4.0"]]}
  :repl-options {:init-ns mirror-parser.core}
  :main ^:skip-aot mirror-parser.core
  :profiles {:uberjar {:aot :all}})
