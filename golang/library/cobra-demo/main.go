package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func main() {
	Execute()
}

func Execute() {
	var rootCmd = &cobra.Command{
		Use:   "ip",
		Short: "IP地址",
		Long:  "服务器IP地址",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("args", args)
		},
	}
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
