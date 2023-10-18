package util

import (
	"strconv"

	"golang.org/x/sys/windows"
)

type DiskMessage struct {
	Name  string
	Free  string
	Total string
}

const (
	B  = 1
	KB = 1024 * B
	MB = 1024 * KB
	GB = 1024 * MB
)

func GetDiskUsage() []DiskMessage {
	DiskList := make([]DiskMessage, 0)

	var free, total, avail uint64

	for _, drive := range "ABCDEFGHIJKLMNOPQRSTUVWXYZ" {
		path := string(drive) + ":\\"
		pathPtr, err := windows.UTF16PtrFromString(path)
		if err != nil {
			continue
		}
		err = windows.GetDiskFreeSpaceEx(pathPtr, &free, &total, &avail)
		if err != nil {
			continue
		}
		free_str := strconv.FormatFloat(float64(free)/float64(GB), 'f', 2, 64) + " GB"
		total_str := strconv.FormatFloat(float64(total)/float64(GB), 'f', 2, 64) + " GB"
		DiskList = append(DiskList, DiskMessage{free_str, total_str, string(drive)})
	}
	return DiskList
}
