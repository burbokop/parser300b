(ns mirror-parser.core
	(:require [clojure.string :as string] 
            [instaparse.core :as insta]
      		  [clojure.tools.cli :refer [parse-opts]]
      		  [clj-yaml.core :as yaml]
            [cheshire.core :as json]) 
    (:gen-class))

(def cli-options
  [["-g" "--grammar [ebnf]" "Grammar"]
   ["-t" "--text [string]" "Text to parse"]
   ["-f" "--format [yaml|json|dump=default]" "Output format"]
   ["-h" "--help"]])

(def cli-err-code -2)
(def gram-parse-err-code -3)
(def text-parse-err-code -4)

(defn usage [options-summary]
  (->> ["EBNF parser"
        ""
        "Usage: lein run [options]"
        ""
        "Options:"
        options-summary
        ]
       (string/join \newline)))

(defn cli-error-msg [errors]
  (str "Can not parse command line args:"
       (string/join \newline errors)))

(defn printfl [arg]
  (print arg)
  (flush)
  )

(defn exit [status msg & {:keys [endl] :or {endl true}}] 
            (let [pr (if endl println printfl)] 
              (if (= status 0)
                (pr msg) 
                (binding [*out* *err*] (pr msg)))) 
            (System/exit status))

(def my-pretty-printer (json/create-pretty-printer 
                        (assoc json/default-pretty-print-options 
                               :indent-arrays? true)))

(defn in?
  "true if coll contains elm"
  [coll elm]
  (some #(= elm %) coll))

(def filter-ws-seq
  (filter (fn [x] (if (vector? x) (not (in? x :whitespace)) true))))

(defn filter-ws [tree]
  (vec (map 
        (fn [x] (if (vector? x) (filter-ws x) x)) 
        (transduce filter-ws-seq conj tree))))

(defn process-result [result, fmt]
  (if (insta/failure? result)
    (exit text-parse-err-code result)
   	(cond
      (= fmt "yaml")
      (exit 0 (yaml/generate-string (filter-ws result) :dumper-options {:flow-style :block}) :endl false)
      (= fmt "json")
      (exit 0 (json/generate-string (filter-ws result) {:pretty my-pretty-printer}) :endl false)
      :else
      (exit 0 result))))

(defn parser
  [grammar]
  (try
    (insta/parser grammar :auto-whitespace :standard)
    (catch Exception err (exit
                         gram-parse-err-code 
                         (format "error parsing grammar:\n>>----------<<\n%s\n>>----------<<\n':\n%s"
                         grammar
                         (ex-message err))))))

(defn -main [& args]
  (let [{:keys [options errors summary]} (parse-opts args cli-options)]
    (cond
      (:help options)
      (exit 0 (usage summary))
      errors
      (exit cli-err-code (cli-error-msg errors))
      (not (:text options))
      (exit cli-err-code "Text not secified")
      (not (:grammar options))
      (exit cli-err-code "Grammar not secified")
      :else
      (process-result
    	((parser (:grammar options)) (:text options) :unhide :all)
  		(:format options)))))

