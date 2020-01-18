// This tool attempts to estimate the curve of the std of ||v|| ~ N(a, b).
// Where a appears to be sqrt(d/3), and b looks logarithmic approaching 0.25.
package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math"
	"math/rand"
	"os"
)

func uniform(r *rand.Rand, n int) []float64 {
	v := make([]float64, n)
	for i := 0; i < n; i++ {
		v[i] = r.Float64()
	}
	return v
}

func dot(a, b []float64) float64 {
	res := 0.0
	for i := range a {
		res += a[i] * b[i]
	}
	return res
}

func norm(a []float64) float64 {
	return math.Sqrt(dot(a, a))
}

func sampleNormalMean(xs []float64) float64 {
	m := 0.0
	for _, x := range xs {
		m += x
	}
	return m / float64(len(xs))
}

func sampleNormalStd(xs []float64) float64 {
	m := sampleNormalMean(xs)
	s := 0.0
	for _, x := range xs {
		t := (x - m)
		s += t * t
	}
	return math.Sqrt(s / float64(len(xs)-1))
}

func normalMLE(xs []float64) (float64, float64) {
	return sampleNormalMean(xs), sampleNormalStd(xs)
}

func main() {
	r := rand.New(rand.NewSource(42))
	var ds []float64
	var ss []float64
	for d := 1; d < 32; d++ {
		var xs []float64
		for i := 0; i < 1e4; i++ {
			v := uniform(r, d)
			xs = append(xs, norm(v))
			_, s := normalMLE(xs)
			if math.IsNaN(s) {
				s = 0.0
			}
			ss = append(ss, s)
		}
	}
	fmt.Println(ss[len(ss)-1])
	out := struct {
		Ds []float64 `json:"ds"`
		Ss []float64 `json:"ss"`
	}{ds, ss}
	f, err := os.Create("ss.json")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	if err := json.NewEncoder(f).Encode(out); err != nil {
		log.Fatal(err)
	}

}
